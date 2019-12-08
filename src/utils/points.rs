use std::cmp::{max, min};
use std::ops::{Add, AddAssign, Mul};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn from_udlr(c: &str) -> Option<Dir> {
        match c {
            "U" => Some(Dir::U),
            "L" => Some(Dir::L),
            "D" => Some(Dir::D),
            "R" => Some(Dir::R),
            _ => None,
        }
    }
    pub fn from_nsew(c: &str) -> Option<Dir> {
        match c {
            "N" => Some(Dir::U),
            "W" => Some(Dir::L),
            "S" => Some(Dir::D),
            "E" => Some(Dir::R),
            _ => None,
        }
    }
    pub fn as_point_delta(self) -> Point {
        match self {
            Dir::U => Point(0, 1),
            Dir::D => Point(0, -1),
            Dir::L => Point(-1, 0),
            Dir::R => Point(1, 0),
        }
    }
    pub fn is_horizontal(self) -> bool {
        self == Dir::R || self == Dir::L
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
    pub fn manhattan(self, other: Self) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub bottom_left: Point,
    pub top_right: Point,
}

impl Aabb {
    pub fn new(p: Point) -> Self {
        Aabb {
            bottom_left: p,
            top_right: p,
        }
    }

    pub fn extend(&self, p: Point) -> Self {
        let mut ans = *self;
        ans.bottom_left.0 = min(ans.bottom_left.0, p.0);
        ans.bottom_left.1 = min(ans.bottom_left.1, p.1);
        ans.top_right.0 = max(ans.top_right.0, p.0);
        ans.top_right.1 = max(ans.top_right.1, p.1);
        ans
    }
    pub fn contains(&self, p: Point) -> bool {
        self.bottom_left.0 <= p.0
            && self.bottom_left.1 <= p.1
            && self.top_right.0 >= p.0
            && self.top_right.1 >= p.1
    }
    pub fn extend_box(&self, b: Self) -> Self {
        self.extend(b.bottom_left).extend(b.top_right)
    }
    pub fn intersect(&self, b: Self) -> Self {
        Aabb {
            bottom_left: Point(
                max(self.bottom_left.0, b.bottom_left.0),
                max(self.bottom_left.1, b.bottom_left.1),
            ),
            top_right: Point(
                min(self.top_right.0, b.top_right.0),
                min(self.top_right.1, b.top_right.1),
            ),
        }
    }
    pub fn width(&self) -> isize {
        self.top_right.0 - self.bottom_left.0
    }
    pub fn height(&self) -> isize {
        self.top_right.1 - self.bottom_left.1
    }
}

#[test]
pub fn bb_tests() {
    let a = Aabb::new(Point(0, 0)).extend(Point(0, 10));
    let b = Aabb::new(Point(-3, 4)).extend(Point(8, 4));
    let i = a.intersect(b);
    println!("{:?}", i);
    assert_eq!(i.bottom_left, Point(0, 4));
    assert_eq!(i.top_right, Point(0, 4));
}
