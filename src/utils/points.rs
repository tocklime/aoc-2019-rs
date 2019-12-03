use parse_display::{Display, FromStr};
use std::cmp::{max, min};
use std::ops::{Add, AddAssign, Mul};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
pub enum Dir {
    U,
    D,
    L,
    R,
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

impl Dir {
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

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Aabb {
    pub fn new(p: Point) -> Self {
        Aabb {
            top_left: p,
            bottom_right: p,
        }
    }

    pub fn extend(&self, p: Point) -> Self {
        let mut ans = self.clone();
        ans.top_left.0 = min(ans.top_left.0, p.0);
        ans.top_left.1 = min(ans.top_left.1, p.1);
        ans.bottom_right.0 = max(ans.bottom_right.0, p.0);
        ans.bottom_right.1 = max(ans.bottom_right.1, p.1);
        ans
    }
    pub fn contains(&self, p: Point) -> bool {
        self.top_left.0 <= p.0
            && self.top_left.1 <= p.1
            && self.bottom_right.0 >= p.0
            && self.bottom_right.1 >= p.1
    }
    pub fn extend_box(&self, b: Self) -> Self {
        self.extend(b.top_left).extend(b.bottom_right)
    }
    pub fn intersect(&self, b: Self) -> Self {
        Aabb {
            top_left: Point(
                max(self.top_left.0, b.top_left.0),
                max(self.top_left.1, b.top_left.1),
            ),
            bottom_right: Point(
                min(self.bottom_right.0, b.bottom_right.0),
                min(self.bottom_right.1, b.bottom_right.1),
            ),
        }
    }
    pub fn width(&self) -> isize {
        self.bottom_right.0 - self.top_left.0
    }
    pub fn height(&self) -> isize {
        self.bottom_right.1 - self.top_left.1
    }
}

#[test]
pub fn bb_tests() {
    let a = Aabb::new(Point(0, 0)).extend(Point(0, 10));
    let b = Aabb::new(Point(-3, 4)).extend(Point(8, 4));
    let i = a.intersect(b);
    println!("{:?}", i);
    assert_eq!(i.top_left, Point(0, 4));
    assert_eq!(i.bottom_right, Point(0, 4));
}
