use super::comp::Computer;
use std::str::FromStr;

pub fn run_with_args(c: &mut Computer, noun: isize, verb: isize) -> isize {
    c.abs_store(1, noun);
    c.abs_store(2, verb);
    c.run().abs_load(0)
}

#[aoc(day2, part1)]
pub fn p1(input: &str) -> isize {
    let mut c = Computer::from_str(input).unwrap();
    run_with_args(&mut c, 12, 2)
}

#[aoc(day2, part2)]
pub fn p2(input: &str) -> isize {
    let mut c = Computer::from_str(input).unwrap();
    for n in 0..100 {
        for v in 0..100 {
            c.reset();
            if run_with_args(&mut c, n, v) == 19_690_720 {
                return 100 * n + v;
            }
        }
    }
    panic!("Cannot find solution for part 2");
}
