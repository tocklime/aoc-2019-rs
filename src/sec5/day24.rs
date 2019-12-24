use std::collections::HashSet;
use num::pow;
use crate::utils::points::Point;
use std::convert::TryInto;
use itertools::iterate;
use crate::utils::algorithms::automata_step;
use std::hash::BuildHasher;

#[aoc_generator(day24)]
pub fn gen(input: &str) -> HashSet<Point> {
    let hm = crate::utils::points::as_point_map(input);
    hm.iter().filter_map(|(a, b)| if b == &'#' { Some(*a) } else { None }).collect()
}

#[aoc(day24, part1)]
pub fn p1<S : BuildHasher + Default + Clone>(input: &HashSet<Point,S>) -> usize {
    let mut seen = HashSet::new();
    iterate(input.clone(),
            |g| automata_step(g, flat_neighbours, lives))
        .map(|x| biodiversity(&x))
        .filter(|&x| !seen.insert(x))
        .nth(0).unwrap()
}

#[aoc(day24, part2)]
pub fn p2<S : BuildHasher + Default>(input: &HashSet<Point, S>) -> usize {
    let btm: HashSet<(Point, i32)> = input.iter().map(|a| (*a, 0)).collect();
    (0..200).fold(btm, |a, _| automata_step(&a, recur_neighbours, lives)).len()
}

pub fn flat_neighbours(p: Point) -> Vec<Point> {
    p.neighbours().iter().cloned().filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < 5 && p.1 < 5).collect()
}

pub fn recur_neighbours(p: (Point, i32)) -> Vec<(Point, i32)> {
    p.0.neighbours().iter().flat_map(move |&n| {
        let mut ans = Vec::with_capacity(4);
        if n.0 < 0 { ans.push((Point(1, 2), p.1 + 1)); }
        if n.1 < 0 { ans.push((Point(2, 1), p.1 + 1)); }
        if n.0 > 4 { ans.push((Point(3, 2), p.1 + 1)); }
        if n.1 > 4 { ans.push((Point(2, 3), p.1 + 1)); }

        if n == Point(2, 2) {
            match p.0 {
                Point(2, 1) => (0..5).for_each(|x| ans.push((Point(x, 0), p.1 - 1))),
                Point(1, 2) => (0..5).for_each(|x| ans.push((Point(0, x), p.1 - 1))),
                Point(3, 2) => (0..5).for_each(|x| ans.push((Point(4, x), p.1 - 1))),
                Point(2, 3) => (0..5).for_each(|x| ans.push((Point(x, 4), p.1 - 1))),
                _ => {}
            }
        }
        if ans.is_empty() {
            ans.push((n, p.1));
        }
        ans
    }).collect()
}

pub fn lives(is_alive: bool, neighbour_count: usize) -> bool {
    neighbour_count == 1 || (!is_alive && neighbour_count == 2)
}


#[test]
pub fn d24p2() {
    assert_eq!(recur_neighbours((Point(0, 0), 0)).len(), 4);
}


pub fn biodiversity<S>(g: &HashSet<Point,S>) -> usize
    where S: BuildHasher
{
    g.iter().map(|&p| {
        pow(2, (p.0 + p.1 * 5).try_into().unwrap())
    }).sum()
}

#[test]
pub fn d24p1() {
    let i = ".....
.....
.....
#....
.#...
";
    let btm = gen(i);
    assert_eq!(biodiversity(&btm), 2129920);
}
