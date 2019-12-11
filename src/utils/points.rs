use num::integer::gcd;
use std::hash::BuildHasher;
use num_enum::TryFromPrimitive;
use std::cmp::{max, min};
use std::convert::TryInto;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
#[derive(PartialEq, Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Dir {
    U,
    L,
    D,
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
    pub fn rotate_left(self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::L => Dir::D,
            Dir::D => Dir::R,
            Dir::R => Dir::U,
        }
    }
    pub fn is_horizontal(self) -> bool {
        self == Dir::R || self == Dir::L
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);
/* impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
    }
} */
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
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Div<isize> for Point {
    type Output = Self;
    fn div(self, rhs: isize) -> Self {
        Point(self.0 / rhs, self.1 / rhs)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl Point {
    pub fn origin() -> Point {
        Point(0, 0)
    }
    pub fn manhattan_from_origin(self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
    pub fn manhattan(self, other: Self) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
    pub fn gcd(self) -> isize {
        gcd(self.0, self.1)
    }
    pub fn size_squared(self) -> isize {
        self.0 * self.0 + self.1 * self.1
    }
    pub fn simplest_direction(self) -> Point {
        self / self.gcd()
    }
    pub fn quadrant_clockwise(self) -> usize {
        match (self.0 >= 0, self.1 >= 0) {
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
            (false, false) => 4,
        }
    }
    pub fn gradient(self) -> f64 {
        self.1 as f64 / self.0 as f64
    }
}
#[derive(Debug)]
pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}
impl PolarCoord {
    pub fn from_point(p: Point) -> PolarCoord {
        PolarCoord {
            r: (p.size_squared() as f64).sqrt(),
            theta: (p.0 as f64).atan2(p.1 as f64),
        }
    }
    pub fn simplify(self) -> PolarCoord {
        PolarCoord {
            r: self.r,
            theta: if self.theta > 2. * PI {
                self.theta % (2. * PI)
            } else {
                self.theta
            },
        }
    }
    pub fn rotate(self, rad: f64) -> PolarCoord {
        PolarCoord {
            r: self.r,
            theta: self.theta + rad,
        }
        .simplify()
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
    pub fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.bottom_left.1..=self.top_right.1)
            .flat_map(move |y| (self.bottom_left.0..=self.top_right.0).map(move |x| Point(x, y)))
    }
    pub fn vec_with<T: Default + Clone>(&self, ft: impl Fn(Point) -> T) -> Vec<Vec<T>> {
        let offset = self.bottom_left;
        let mut v = vec![vec![Default::default(); self.width()]; self.height()];
        for p in self.all_points() {
            let rel = p - offset;
            v[rel.1 as usize][rel.0 as usize] = ft(p);
        }
        v
    }
    pub fn width(&self) -> usize {
        (1 + self.top_right.0 - self.bottom_left.0)
            .try_into()
            .unwrap()
    }
    pub fn height(&self) -> usize {
        (1 + self.top_right.1 - self.bottom_left.1)
            .try_into()
            .unwrap()
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

use std::collections::HashMap;
pub fn point_map_bounding_box<T, S : BuildHasher>(hm: &HashMap<Point, T, S>) -> Aabb {
    let a_point = hm.keys().nth(0).unwrap();
    hm.keys().fold(Aabb::new(*a_point), |bb, &k| bb.extend(k))
}
pub fn point_hashmap_to_array<T: Default + Copy, S : BuildHasher>(hm: &HashMap<Point, T, S>) -> Vec<Vec<T>> {
    let bb = hm
        .keys()
        .fold(Aabb::new(Point::origin()), |bb, &k| bb.extend(k));
    let mut o: Vec<Vec<T>> = vec![vec![Default::default(); bb.width()]; bb.height()];
    let offset = bb.bottom_left;
    for p in bb.all_points() {
        let rel = p + offset;
        o[rel.1 as usize][rel.0 as usize] = hm.get(&p).cloned().unwrap_or_default();
    }
    o
}
pub fn render_char_map<S : BuildHasher>(m: &HashMap<Point, char, S>) -> String {
    let bb = crate::utils::points::point_map_bounding_box(&m);
    let v = bb.vec_with(|p| *m.get(&p).unwrap_or(&' '));
    v.iter()
        .map(|l| "\n".to_string() + &l.iter().collect::<String>())
        .rev() //looks upside down...
        .collect()
}
