use std::fmt;

use super::computer::Computer;
use super::enums::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arg(isize, ParameterMode);

impl Arg {
    pub fn new(value: isize, mode: ParameterMode) -> Arg {
        Arg(value, mode)
    }
    pub fn get(self, c: &Computer) -> isize {
        match self {
            Arg(i, ParameterMode::IMMEDIATE) => i,
            Arg(i, ParameterMode::POSITION) => c.abs_load(i),
            Arg(i, ParameterMode::RELATIVE) => c.rel_load(i),
        }
    }
    pub fn ptr(self, c: &Computer) -> isize {
        match self {
            Arg(i, ParameterMode::IMMEDIATE) => i,
            Arg(i, ParameterMode::POSITION) => i,
            Arg(i, ParameterMode::RELATIVE) => c.rel_offset(i),
        }
    }
}
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            ParameterMode::IMMEDIATE => write!(f, " {: <4}", self.0),
            ParameterMode::POSITION => write!(f, "@{: <4}", self.0),
            ParameterMode::RELATIVE => write!(f, "R{: <4}", self.0),
        }
    }
}
