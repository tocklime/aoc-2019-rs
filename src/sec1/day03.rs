use crate::utils::points::{Aabb, Dir, Point};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WireLine {
    pub dir: Dir,
    pub len: usize,
    pub start: Point,
    pub signal_delay: usize,
}

impl WireLine {
    pub fn end(self) -> Point {
        self.start + self.dir.as_point_delta() * self.len
    }
    pub fn as_bb(self) -> Aabb {
        Aabb::new(self.start).extend(self.end())
    }
    pub fn intersects(self, other: &Self) -> Option<Point> {
        let bb1 = self.as_bb();
        let bb2 = other.as_bb();
        let joined = bb1.extend_box(bb2);
        if joined.height() > (bb1.height() + bb2.height()) {
            return None;
        }
        if joined.width() > (bb1.width() + bb2.width()) {
            return None;
        }
        let a = bb1.intersect(bb2);
        Some(a.bottom_left)
    }
    pub fn signal_delay_at(self, p: Point) -> usize {
        self.signal_delay + p.manhattan(self.start)
    }
}

type Wire = Vec<WireLine>;

#[aoc_generator(day3, fast)]
pub fn gen(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|l| {
            let mut p = Point(0, 0);
            let mut delay = 0;
            l.split(',')
                .map(|i| {
                    let (d, n) = i.split_at(1);
                    let wl = WireLine {
                        dir: Dir::from_udlr(d).unwrap(),
                        len: n.parse().unwrap(),
                        start: p,
                        signal_delay: delay,
                    };
                    delay += wl.len;
                    p += wl.dir.as_point_delta() * wl.len;
                    wl
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1, fast)]
pub fn p1(input: &[Wire]) -> usize {
    input[0]
        .iter()
        .flat_map(move |a| input[1].iter().filter_map(move |b| a.intersects(b)))
        .map(Point::manhattan_from_origin)
        .filter(|l| *l > 0)
        .min()
        .unwrap()
}

#[aoc(day3, part2, fast)]
pub fn p2(input: &[Wire]) -> usize {
    input[0]
        .iter()
        .flat_map(move |a| {
            input[1].iter().filter_map(move |b| {
                a.intersects(b)
                    .map(|i| a.signal_delay_at(i) + b.signal_delay_at(i))
            })
        })
        .filter(|l| *l > 0)
        .min()
        .unwrap()
}
