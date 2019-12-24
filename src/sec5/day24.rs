use std::collections::{BTreeSet, HashSet};
use num::pow;
use std::hash::Hash;
use crate::utils::points::Point;
use std::convert::TryInto;

#[aoc_generator(day24)]
pub fn gen(input: &str) -> BTreeSet<Point> {
    let hm = crate::utils::points::as_point_map(input);
    hm.iter().filter_map(|(a, b)| if b == &'#' { Some(*a) } else { None }).collect()
}

#[aoc(day24, part1)]
pub fn p1(input: &BTreeSet<Point>) -> usize {
    let mut seen = HashSet::new();
    let mut grid = input.clone();
    loop {
        if !seen.insert(grid.clone()) {
            break biodiversity(&grid);
        }
        grid = step(&grid, flat_neighbours, lives);
    }
}

#[aoc(day24, part2)]
pub fn p2(input: &BTreeSet<Point>) -> usize {
    let btm: BTreeSet<(Point, i32)> = input.iter().map(|a| (*a, 0)).collect();
    (0..200).fold(btm, |a, _| step(&a, recur_neighbours, lives)).len()
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

pub fn step<T, FN, FC>(g: &BTreeSet<T>, neighbours: FN, check: FC) -> BTreeSet<T>
    where FN: Fn(T) -> Vec<T>,
          FC: Fn(bool, usize) -> bool,
          T: Ord + Copy + Hash
{
    let candidates: HashSet<T> = g.iter().flat_map(|&x| neighbours(x)).collect();
    candidates.iter()
        .cloned()
        .filter(|p| {
            let n = neighbours(*p).iter().filter(|n| g.contains(n)).count();
            check(g.contains(p), n)
        }).collect()
}

#[test]
pub fn d24p2() {
    assert_eq!(recur_neighbours((Point(0, 0), 0)).len(), 4);
}


pub fn biodiversity(g: &BTreeSet<Point>) -> usize {
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
