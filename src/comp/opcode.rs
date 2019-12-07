use num_enum::TryFromPrimitive;
use std::fmt;

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

impl OpCode {
    pub fn arg_count(self) -> usize {
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
