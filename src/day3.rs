use super::utils::points::{Dir, Point};
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WireLine {
    dir: Dir,
    len: usize,
}

type Wire = Vec<WireLine>;

pub fn points<'a>(w: &'a Wire) -> impl Iterator<Item = Point> + 'a {
    let mut p = Point(0, 0);
    w.iter().flat_map(move |wl| {
        let pd = wl.dir.as_point_delta();
        let n = (1..=wl.len).map(move |i| p + (pd * i));
        p += pd * wl.len;
        return n;
    })
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
pub fn p1(input: &[Wire]) -> usize {
    let w1_set: HashSet<_> = points(&input[0]).collect();
    points(&input[1])
        .filter(|p| w1_set.contains(p))
        .map(|p| p.manhattan_from_origin())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn p2(input: &[Wire]) -> usize {
    let mut d: HashMap<Point, usize> = HashMap::new();
    for (p, i) in points(&input[0]).zip(1..) {
        d.entry(p).and_modify(|e| *e = min(*e, i)).or_insert(i);
    }
    points(&input[1])
        .zip(1..)
        .filter(|(p, _)| d.contains_key(&p))
        .map(|(p, l)| d[&p] + l)
        .min()
        .unwrap()
}

#[test]
pub fn tests() {
    let h0 = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let h1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let h2 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    //p1
    assert_eq!(p1(&gen(h0)), 6);
    assert_eq!(p1(&gen(h1)), 159);
    assert_eq!(p1(&gen(h2)), 135);
    //p2
    assert_eq!(p2(&gen(h0)), 30);
    assert_eq!(p2(&gen(h1)), 610);
    assert_eq!(p2(&gen(h2)), 410);
}
