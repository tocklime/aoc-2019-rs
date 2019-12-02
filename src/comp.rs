use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub enum ComputerState {
    RUNNING,
    HALTED,
}

#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum OpCode {
    Add = 1,
    Mult = 2,
    Halt = 99,
}

impl OpCode {
    fn arg_count(self) -> usize {
        match self {
            OpCode::Add | OpCode::Mult => 3,
            OpCode::Halt => 0,
        }
    }
    fn execute(&self, c: &mut Computer) {
        match self {
            OpCode::Add => c.abs_store(c.load(3), c.abs_load(c.load(1)) + c.abs_load(c.load(2))),
            OpCode::Mult => c.abs_store(c.load(3), c.abs_load(c.load(1)) * c.abs_load(c.load(2))),
            OpCode::Halt => c.state = ComputerState::HALTED,
        }
        c.inc_ip(1 + self.arg_count())
    }
}

#[derive(Debug)]
pub struct Computer<'a> {
    initial_mem: &'a [usize],
    memory: Vec<usize>,
    instruction_pointer: usize,
    state: ComputerState,
}

impl Computer<'_> {
    pub fn new<'a>(initial_mem: &[usize]) -> Computer {
        let mut c = Computer {
            initial_mem: initial_mem.clone(),
            memory: vec![0; initial_mem.len()],
            instruction_pointer: 0,
            state: ComputerState::RUNNING,
        };
        c.reset();
        return c;
    }
    pub fn reset(&mut self) -> &Self {
        self.memory = self.initial_mem.iter().cloned().collect();
        self.instruction_pointer = 0;
        self.state = ComputerState::RUNNING;
        return self;
    }
    pub fn current_op(&self) -> OpCode {
        let as_int = self.memory[self.instruction_pointer];
        match OpCode::try_from(as_int) {
            Ok(x) => return x,
            Err(_) => panic!(
                "Unknown op code @ {}: {}\n\n{:?}",
                self.instruction_pointer, as_int, self
            ),
        }
    }
    pub fn abs_load(&self, pos: usize) -> usize {
        self.memory[pos]
    }
    pub fn load(&self, offset: usize) -> usize {
        self.memory[self.instruction_pointer + offset]
    }
    pub fn store(&mut self, offset: usize, value: usize) {
        self.memory[self.instruction_pointer + offset] = value;
    }
    pub fn abs_store(&mut self, offset: usize, value: usize) {
        self.memory[offset] = value;
    }
    pub fn inc_ip(&mut self, offset: usize) {
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
        self.current_op().execute(self);
        return self;
    }
}
