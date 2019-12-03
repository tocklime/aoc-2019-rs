use parse_display::{Display, FromStr};
use std::ops::{Add, AddAssign, Mul};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
pub enum Dir {
    U,
    D,
    L,
    R,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point(pub isize, pub isize);

impl Mul<isize> for Point {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        Point(self.0 * rhs, self.1 * rhs)
    }
}
impl Mul<usize> for Point {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        self * (rhs as isize)
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl Point {
    pub fn manhattan_from_origin(self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
}

impl Dir {
    pub fn as_point_delta(self) -> Point {
        match self {
            Dir::U => Point(0, 1),
            Dir::D => Point(0, -1),
            Dir::L => Point(-1, 0),
            Dir::R => Point(1, 0),
        }
    }
}
