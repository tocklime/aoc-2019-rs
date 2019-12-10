use std::cmp::Ord;
use std::convert::{From, TryInto};
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

pub trait CompMem:
    Ord
    + Add<Output = Self>
    + Mul<Output = Self>
    + Display
    + Clone
    + Default
    + TryInto<isize>
    + From<bool>
    + Debug
    + Copy
{
    fn as_isize(self) -> isize {
        self.try_into()
            .unwrap_or_else(|_| panic!("Cannot convert memory to isize"))
    }
}

impl CompMem for isize {}
impl CompMem for i8 {}
impl CompMem for i16 {}
impl CompMem for i32 {}
impl CompMem for i64 {}
