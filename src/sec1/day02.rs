use crate::comp::Computer;
use std::str::FromStr;

pub fn run_with_args(c: &mut Computer, noun: i32, verb: i32) -> i32 {
    c.abs_store(1, noun);
    c.abs_store(2, verb);
    c.run().abs_load(0)
}

#[aoc(day2, part1)]
pub fn p1(input: &str) -> i32 {
    let mut c = Computer::from_str(input).unwrap();
    run_with_args(&mut c, 12, 2)
}

#[aoc(day2, part2)]
pub fn p2(input: &str) -> i32 {
    let mut c = Computer::from_str(input).unwrap();
    (0..100)
        .flat_map(move |n| (0..100).map(move |v| (n, v)))
        .filter(|(n, v)| run_with_args(c.reset(), *n, *v) == 19_690_720)
        .map(|(n, v)| 100 * n + v)
        .nth(0)
        .unwrap()
}
