#[derive(Clone)]
pub enum ComputerState {
    RUNNING,
    HALTED,
}

pub struct Computer<'a> {
    initial_mem: &'a [usize],
    memory: Vec<usize>,
    pub instruction_pointer: usize,
    pub state: ComputerState,
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
    pub fn current_op(&self) -> usize {
        self.memory[self.instruction_pointer]
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
            match self.step() {
                ComputerState::HALTED => return self,
                ComputerState::RUNNING => (),
            }
        }
    }
    pub fn step(&mut self) -> &ComputerState {
        match self.current_op() {
            // ADD
            1 => {
                self.abs_store(
                    self.load(3),
                    self.abs_load(self.load(1)) + self.abs_load(self.load(2)),
                );
                self.inc_ip(4)
            }
            // MULT
            2 => {
                self.abs_store(
                    self.load(3),
                    self.abs_load(self.load(1)) * self.abs_load(self.load(2)),
                );
                self.inc_ip(4);
            }
            99 => self.state = ComputerState::HALTED,
            // PANIC
            _ => panic!(
                "Unknown OP Code {} at IP {}",
                self.current_op(),
                self.instruction_pointer
            ),
        }
        return &self.state;
    }
}
