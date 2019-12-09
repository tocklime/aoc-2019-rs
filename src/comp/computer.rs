use log::info;
use std::cmp;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use super::enums::*;
use super::oparg::Arg;
use super::opcode::OpCode;
#[derive(Debug)]
pub struct Computer {
    name: String,
    initial_mem: Vec<isize>,
    memory: HashMap<isize, isize>,
    instruction_pointer: isize,
    state: ComputerState,
    fixed_input: Vec<isize>,
    input_chan: Option<Receiver<isize>>,
    output: Vec<isize>,
    output_chan: Option<Sender<isize>>,
    relative_base: isize,
}

impl FromStr for Computer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is: Result<Vec<_>, _> = s.split(',').map(|x| x.parse::<isize>()).collect();
        Ok(Computer::new(&is?))
    }
}
impl Computer {
    pub fn new(initial_mem: &[isize]) -> Self {
        let mut c = Computer {
            initial_mem: Vec::from(initial_mem),
            name: String::from("COMP"),
            memory: HashMap::new(),
            instruction_pointer: 0,
            state: ComputerState::RUNNING,
            fixed_input: vec![],
            input_chan: None,
            output: vec![],
            output_chan: None,
            relative_base: 0,
        };
        c.reset();
        c
    }
    pub fn with_name(&mut self, n: String) -> &mut Self {
        self.name = n;
        self
    }
    pub fn disassembly(&self) -> String {
        let mut ip = 0;
        let mut output = String::new();
        let max_mem: usize = cmp::max(
            self.initial_mem.len(),
            *self.memory.keys().max().unwrap_or(&0) as usize,
        );
        while ip < max_mem {
            let a = self.get_args(ip);
            match Op::try_from_mem_slice(&a) {
                Some(o) => {
                    output.push_str(&format!("{: >4}: {}\n", ip, o));
                    ip += 1 + o.op.arg_count();
                }
                None => {
                    for i in &a {
                        output.push_str(&format!("{: >4}: {}\n", ip, i));
                        ip += 1;
                    }
                }
            }
        }
        output
    }

    pub fn get_args(&self, ip: usize) -> [isize; 4] {
        let mut ans: [isize; 4] = Default::default();
        for (i, a) in ans.iter_mut().enumerate() {
            *a = self.abs_load((ip + i) as isize);
        }
        ans
    }
    pub fn get_last_output(&self) -> isize {
        *self.get_output().last().unwrap()
    }
    pub fn get_output(&self) -> &[isize] {
        &self.output
    }
    pub fn with_input(&mut self, x: isize) -> &mut Self {
        self.fixed_input.push(x);
        self
    }
    pub fn connect_output_from(&mut self, other: &mut Self, initial_input: &[isize]) -> &mut Self {
        let (tx, rx) = mpsc::channel::<isize>();
        for &v in initial_input {
            tx.send(v).expect("Failed to send initial value");
        }
        other.with_chan_output(tx);
        self.with_chan_input(rx)
    }
    pub fn with_chan_input(&mut self, x: Receiver<isize>) -> &mut Self {
        self.input_chan = Some(x);
        self
    }
    pub fn with_chan_output(&mut self, x: Sender<isize>) -> &mut Self {
        self.output_chan = Some(x);
        self
    }
    pub fn reset(&mut self) -> &mut Self {
        self.memory = HashMap::new();
        self.instruction_pointer = 0;
        self.state = ComputerState::RUNNING;
        self.relative_base = 0;
        self.fixed_input = vec![];
        self
    }
    pub fn current_op_with_args(&self) -> Op {
        let ms = self.get_args(self.instruction_pointer as usize);
        Op::from_mem_slice(&ms)
    }
    pub fn abs_load(&self, pos: isize) -> isize {
        *self.memory.get(&pos).unwrap_or_else(|| {
            if pos >= 0 && (pos as usize) < self.initial_mem.len() {
                &self.initial_mem[pos as usize]
            } else {
                &0
            }
        })
    }
    pub fn rel_load(&self, offset: isize) -> isize {
        let a = self.abs_load(self.relative_base + offset);
        info!(
            "RELLOAD {} + {} ({}) = {}",
            self.relative_base,
            offset,
            self.relative_base + offset,
            a
        );
        a
    }
    pub fn rel_offset(&self, offset: isize) -> isize {
        self.relative_base + offset
    }
    pub fn load(&self, offset: isize) -> isize {
        self.abs_load(self.instruction_pointer + offset)
    }
    pub fn store(&mut self, offset: isize, value: isize) {
        self.abs_store(self.instruction_pointer + offset, value)
    }
    pub fn abs_store(&mut self, offset: isize, value: isize) {
        info!("STORE @{} = {}", offset, value);
        *self.memory.entry(offset).or_insert(0) = value;
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
    pub fn step(&mut self) -> &mut Self {
        self.current_op_with_args().execute(self);
        self
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
        let op1 = ps % 10;
        let op2 = (ps / 10) % 10;
        let op3 = (ps / 100) % 10;
        let o = Some(Op {
            op: OpCode::try_from(m[0] % 100).ok()?,
            args: [
                Arg::new(m[1], ParameterMode::try_from(op1).ok()?),
                Arg::new(m[2], ParameterMode::try_from(op2).ok()?),
                Arg::new(m[3], ParameterMode::try_from(op3).ok()?),
            ],
        });
        info!("E: {}\n", o.unwrap());
        o
    }
    pub fn from_mem_slice(m: &[isize; 4]) -> Op {
        Op::try_from_mem_slice(m).unwrap()
    }
    pub fn execute(&self, c: &mut Computer) {
        let op_count = self.op.arg_count();
        let ps = self.args;
        let mut do_ip_inc = true;
        match self.op {
            OpCode::Add => c.abs_store(ps[2].ptr(c), ps[0].get(c) + ps[1].get(c)),
            OpCode::Mult => c.abs_store(ps[2].ptr(c), ps[0].get(c) * ps[1].get(c)),
            OpCode::LessThan => c.abs_store(ps[2].ptr(c), (ps[0].get(c) < ps[1].get(c)).into()),
            OpCode::Equals => c.abs_store(ps[2].ptr(c), (ps[0].get(c) == ps[1].get(c)).into()),
            OpCode::Input => {
                let i = if !c.fixed_input.is_empty() {
                    c.fixed_input.remove(0)
                } else if let Some(r) = &c.input_chan {
                    info!(target: "IO", "{} INP WAIT", c.name);
                    r.recv().expect("No value on receiver")
                } else {
                    panic!("No input")
                };
                info!(target: "IO", "{} INP --> {}", c.name, i);
                info!("INP --> {}, {:?}", i, ps);
                c.abs_store(ps[0].ptr(c), i);
            }
            OpCode::Output => {
                let o = ps[0].get(c);
                info!("OUT: {}", o);
                c.output.push(o);
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
            OpCode::MoveRelativeBase => {
                c.relative_base += ps[0].get(c);
                info!("RELBASE NOW {}", c.relative_base);
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
        info!("IP = {}", c.instruction_pointer);
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
