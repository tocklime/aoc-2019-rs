use std::fmt;

use super::compmem::CompMem;
use super::computer::Computer;
use super::enums::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arg<MemType>(MemType, ParameterMode);

impl<MemType> Arg<MemType>
where
    MemType: CompMem,
{
    pub fn new(value: MemType, mode: ParameterMode) -> Self {
        Arg(value, mode)
    }
    pub fn get(self, c: &Computer<MemType>) -> MemType {
        match self {
            Arg(i, ParameterMode::IMMEDIATE) => i,
            Arg(i, ParameterMode::POSITION) => c.abs_load(i.as_isize()),
            Arg(i, ParameterMode::RELATIVE) => c.rel_load(i.as_isize()),
        }
    }
    pub fn ptr(self, c: &Computer<MemType>) -> isize {
        match self {
            Arg(i, ParameterMode::IMMEDIATE) => i.as_isize(),
            Arg(i, ParameterMode::POSITION) => i.as_isize(),
            Arg(i, ParameterMode::RELATIVE) => c.rel_offset(i.as_isize()),
        }
    }
}
impl<MemType> fmt::Display for Arg<MemType>
where
    MemType: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            ParameterMode::IMMEDIATE => write!(f, " {: <4}", self.0),
            ParameterMode::POSITION => write!(f, "@{: <4}", self.0),
            ParameterMode::RELATIVE => write!(f, "R{: <4}", self.0),
        }
    }
}
