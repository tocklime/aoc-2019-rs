use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{}")]
pub enum Dir {
    #[display("U")]
    U,
    #[display("D")]
    D,
    #[display("L")]
    L,
    #[display("R")]
    R,
}

#[display("{dir}{len}")]
#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
pub struct WireLine {
    dir: Dir,
    len: usize,
}

type Wire = Vec<WireLine>;

pub fn points(w: &Wire) -> Vec<(isize, isize)> {
    let mut x = 0_isize;
    let mut y = 0_isize;
    w.iter()
        .flat_map(|wl| {
            let (xd, yd): (isize, isize) = match wl.dir {
                Dir::U => (0, 1),
                Dir::D => (0, -1),
                Dir::L => (-1, 0),
                Dir::R => (1, 0),
            };
            let l = wl.len as isize;
            let n = (1..=l).map(move |i| (x + xd * i, y + yd * i));
            x += xd * l;
            y += yd * l;
            return n;
        })
        .collect()
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|i| {
                    let (d, n) = i.split_at(1);
                    WireLine {
                        dir: d.parse().unwrap(),
                        len: n.parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn p1(input: &[Wire]) -> isize {
    let w1_points = points(&input[0]);
    let w1_set = w1_points.iter().collect::<HashSet<_>>();
    let w2_points = points(&input[1]);
    let w2_set = w2_points.iter().collect::<HashSet<_>>();
    let i_sect = w1_set.intersection(&w2_set);
    i_sect.map(|(a, b)| (a.abs() + b.abs())).min().unwrap()
}

pub fn mk_dict(w: &Wire) -> HashMap<(isize, isize), usize> {
    let mut d: HashMap<(isize, isize), usize> = HashMap::new();
    for (p, i) in points(w).iter().zip(1..) {
        d.entry(*p)
            .and_modify(|e| *e = std::cmp::min(*e, i))
            .or_insert(i);
    }
    return d;
}

#[aoc(day3, part2)]
pub fn p2(input: &[Wire]) -> usize {
    let d1 = mk_dict(&input[0]);
    let d2 = mk_dict(&input[1]);
    let ks1 = d1.keys().collect::<HashSet<_>>();
    let ks2 = d2.keys().collect::<HashSet<_>>();
    ks1.intersection(&ks2).map(|k| d1[k] + d2[k]).min().unwrap()
}

#[test]
pub fn tests() {
    let h0 = "R8,U5,L5,D3\nU7,R6,D4,L4";
    assert_eq!(p1(&gen(h0)), 6);
    let h1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    assert_eq!(p1(&gen(h1)), 159);
    //p2
    assert_eq!(p2(&gen(h0)), 30);
}
