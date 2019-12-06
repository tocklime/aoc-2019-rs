use num_enum::TryFromPrimitive;
use std::cmp::min;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

mod comp_tests;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputerState {
    RUNNING,
    HALTED,
}

#[derive(Clone, Copy, TryFromPrimitive, PartialEq, Eq, Debug)]
#[repr(isize)]
pub enum ParameterMode {
    POSITION = 0,
    IMMEDIATE = 1,
}

#[derive(Clone, Copy, TryFromPrimitive, PartialEq, Eq, Debug)]
#[repr(isize)]
pub enum OpCode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}
impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OpCode::Add => "Add",
            OpCode::Mult => "Mult",
            OpCode::Input => "Input",
            OpCode::Output => "Output",
            OpCode::JumpIfTrue => "Jump If True",
            OpCode::JumpIfFalse => "Jump If False",
            OpCode::LessThan => "Less Than",
            OpCode::Equals => "Equals",
            OpCode::Halt => "Halt",
        };
        write!(f, "{: <20}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Op {
    op: OpCode,
    args: [Arg; 3],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arg(isize, ParameterMode);

impl Arg {
    pub fn imm(self) -> isize {
        self.0
    }
    pub fn get(self, c: &Computer) -> isize {
        match self {
            Arg(i, ParameterMode::IMMEDIATE) => i,
            Arg(i, ParameterMode::POSITION) => c.abs_load(i),
        }
    }
}
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            ParameterMode::IMMEDIATE => write!(f, " {: <4}", self.0),
            ParameterMode::POSITION => write!(f, "@{: <4}", self.0),
        }
    }
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
                Arg(m[1], ParameterMode::try_from(op1).ok()?),
                Arg(m[2], ParameterMode::try_from(op2).ok()?),
                Arg(m[3], ParameterMode::try_from(op3).ok()?),
            ],
        })
    }
    pub fn from_mem_slice(m: &[isize; 4]) -> Op {
        Op::try_from_mem_slice(m).unwrap()
    }
    fn execute(&self, c: &mut Computer) {
        let op_count = self.op.arg_count();
        let ps = self.args;
        let mut do_ip_inc = true;
        match self.op {
            OpCode::Add => c.abs_store(ps[2].imm(), ps[0].get(c) + ps[1].get(c)),
            OpCode::Mult => c.abs_store(ps[2].imm(), ps[0].get(c) * ps[1].get(c)),
            OpCode::Input => c.abs_store(ps[0].imm(), c.input),
            OpCode::Output => c.output = ps[0].get(c),
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                let cond = ps[0].get(c);
                if (cond != 0) == (self.op == OpCode::JumpIfTrue) {
                    c.instruction_pointer = ps[1].get(c);
                    do_ip_inc = false;
                }
            }
            OpCode::LessThan => {
                let p1 = ps[0].get(c);
                let p2 = ps[1].get(c);
                c.abs_store(ps[2].imm(), if p1 < p2 { 1 } else { 0 });
            }
            OpCode::Equals => {
                let p1 = ps[0].get(c);
                let p2 = ps[1].get(c);
                c.abs_store(ps[2].imm(), if p1 == p2 { 1 } else { 0 });
            }
            OpCode::Halt => {
                c.state = ComputerState::HALTED;
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

impl OpCode {
    fn arg_count(self) -> usize {
        match self {
            OpCode::Add | OpCode::Mult => 3,
            OpCode::Input => 1,
            OpCode::Output => 1,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
            OpCode::LessThan | OpCode::Equals => 3,
            OpCode::Halt => 0,
        }
    }
}

#[derive(Debug)]
pub struct Computer<'a> {
    initial_mem: &'a [isize],
    memory: Vec<isize>,
    instruction_pointer: isize,
    state: ComputerState,
    input: isize,
    output: isize,
}

impl Computer<'_> {
    pub fn new<'a>(initial_mem: &[isize]) -> Computer {
        let mut c = Computer {
            initial_mem: initial_mem.clone(),
            memory: vec![0; initial_mem.len()],
            instruction_pointer: 0,
            state: ComputerState::RUNNING,
            input: Default::default(),
            output: Default::default(),
        };
        c.reset();
        return c;
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
        self.output
    }
    pub fn with_input(&mut self, x: isize) -> &Self {
        self.input = x;
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
                ComputerState::HALTED => return self,
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
