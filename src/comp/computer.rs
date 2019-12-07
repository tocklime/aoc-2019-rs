use log::info;
use std::cmp::min;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::mpsc::{Receiver, Sender};

use super::enums::*;
use super::inputmode::InputMode;
use super::oparg::Arg;
use super::opcode::OpCode;
#[derive(Debug)]
pub struct Computer {
    name: String,
    initial_mem: Vec<isize>,
    memory: Vec<isize>,
    instruction_pointer: isize,
    state: ComputerState,
    input: InputMode,
    last_output: isize,
    output_chan: Option<Sender<isize>>,
}

impl Computer {
    pub fn new<'b>(initial_mem: &'b [isize]) -> Computer {
        let mut c = Computer {
            initial_mem: Vec::from(initial_mem),
            name: String::from("COMP"),
            memory: vec![0; initial_mem.len()],
            instruction_pointer: 0,
            state: ComputerState::RUNNING,
            input: InputMode::List(vec![]),
            last_output: 0,
            output_chan: None,
        };
        c.reset();
        return c;
    }
    pub fn with_name(&mut self, n: String) -> &mut Self {
        self.name = n;
        self
    }
    pub fn disassembly(&self) -> String {
        let mut ip = 0;
        let mut output = String::new();
        while ip < self.memory.len() {
            let a = self.get_args(ip);
            match Op::try_from_mem_slice(&a) {
                Some(o) => {
                    output.push_str(&format!("{: >4}: {}\n", ip, o));
                    ip += 1 + o.op.arg_count();
                }
                None => {
                    for i in 0..=3 {
                        output.push_str(&format!("{: >4}: {}\n", ip, a[i]));
                        ip += 1;
                    }
                }
            }
        }
        output
    }

    pub fn get_args(&self, ip: usize) -> [isize; 4] {
        let mut ans: [isize; 4] = Default::default();
        let end = min(ip + 4, self.memory.len());
        let mem_slice = &self.memory[ip..end];
        for i in 0..mem_slice.len() {
            ans[i] = mem_slice[i];
        }
        ans
    }
    pub fn get_output(&self) -> isize {
        self.last_output
    }
    pub fn with_input(&mut self, x: isize) -> &mut Self {
        match &mut self.input {
            InputMode::List(l) => l.push(x),
            InputMode::Channel(_) => unimplemented!(),
        }
        self
    }
    pub fn with_chan_input(&mut self, x: Receiver<isize>) -> &mut Self {
        self.input = InputMode::Channel(x);
        self
    }
    pub fn with_chan_output(&mut self, x: Sender<isize>) -> &mut Self {
        self.output_chan = Some(x);
        self
    }
    pub fn reset(&mut self) -> &Self {
        self.memory = self.initial_mem.iter().cloned().collect();
        self.instruction_pointer = 0;
        self.state = ComputerState::RUNNING;
        return self;
    }
    pub fn current_op_with_args(&self) -> Op {
        let ms = self.get_args(self.instruction_pointer as usize);
        Op::from_mem_slice(&ms)
    }
    pub fn abs_load(&self, pos: isize) -> isize {
        let as_u: usize = pos.try_into().expect("Bad memory location");
        self.memory[as_u]
    }
    pub fn load(&self, offset: isize) -> isize {
        self.abs_load(self.instruction_pointer + offset)
    }
    pub fn store(&mut self, offset: isize, value: isize) {
        self.abs_store(self.instruction_pointer + offset, value)
    }
    pub fn abs_store(&mut self, offset: isize, value: isize) {
        let as_u: usize = offset.try_into().expect("Negative memory location");
        self.memory[as_u] = value;
    }
    pub fn inc_ip(&mut self, offset: isize) {
        self.instruction_pointer += offset;
    }
    pub fn run(&mut self) -> &Self {
        loop {
            match self.step().state {
                ComputerState::HALTED => {
                    return self;
                }
                ComputerState::RUNNING => (),
            }
        }
    }
    pub fn step(&mut self) -> &Self {
        self.current_op_with_args().execute(self);
        return self;
    }
    pub fn state(&self) -> ComputerState {
        self.state
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Op {
    op: OpCode,
    args: [Arg; 3],
}

impl Op {
    pub fn try_from_mem_slice(m: &[isize; 4]) -> Option<Op> {
        let ps = m[0] / 100;
        let op1 = (ps / 1) % 10;
        let op2 = (ps / 10) % 10;
        let op3 = (ps / 100) % 10;
        Some(Op {
            op: OpCode::try_from(m[0] % 100).ok()?,
            args: [
                Arg::new(m[1], ParameterMode::try_from(op1).ok()?),
                Arg::new(m[2], ParameterMode::try_from(op2).ok()?),
                Arg::new(m[3], ParameterMode::try_from(op3).ok()?),
            ],
        })
    }
    pub fn from_mem_slice(m: &[isize; 4]) -> Op {
        Op::try_from_mem_slice(m).unwrap()
    }
    pub fn execute(&self, c: &mut Computer) {
        let op_count = self.op.arg_count();
        let ps = self.args;
        let mut do_ip_inc = true;
        match self.op {
            OpCode::Add => c.abs_store(ps[2].imm(), ps[0].get(c) + ps[1].get(c)),
            OpCode::Mult => c.abs_store(ps[2].imm(), ps[0].get(c) * ps[1].get(c)),
            OpCode::LessThan => c.abs_store(ps[2].imm(), (ps[0].get(c) < ps[1].get(c)).into()),
            OpCode::Equals => c.abs_store(ps[2].imm(), (ps[0].get(c) == ps[1].get(c)).into()),
            OpCode::Input => {
                info!(target: "IO", "{} INP WAIT", c.name);
                let i = match &mut c.input {
                    InputMode::List(x) => x.remove(0),
                    InputMode::Channel(r) => r.recv().expect("No value on receiver"),
                };
                info!(target: "IO", "{} INP --> {}", c.name, i);
                c.abs_store(ps[0].imm(), i);
            }
            OpCode::Output => {
                let o = ps[0].get(c);
                c.last_output = o;
                if let Some(ch) = &c.output_chan {
                    info!(target: "IO", "{} OUT <-- {}", c.name, o);
                    ch.send(o).expect("Could not send");
                }
            }
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                if (ps[0].get(c) != 0) == (self.op == OpCode::JumpIfTrue) {
                    c.instruction_pointer = ps[1].get(c);
                    do_ip_inc = false;
                }
            }
            OpCode::Halt => {
                c.state = ComputerState::HALTED;
                info!("{} HALTED", c.name);
                do_ip_inc = false;
            }
        }
        if do_ip_inc {
            c.inc_ip((1 + op_count).try_into().unwrap());
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{: <20}", self.op)?;
        for i in 0..self.op.arg_count() {
            write!(f, "{: <5} ", self.args[i])?;
        }
        fmt::Result::Ok(())
    }
}
