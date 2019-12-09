use super::comp::Computer;
use std::str::FromStr;

#[aoc(day5, part1)]
pub fn p1(input: &str) -> isize {
    let mut c = Computer::from_str(input).unwrap();
    c.with_input(1).run().get_last_output()
}

#[aoc(day5, part2)]
pub fn p2(input: &str) -> isize {
    let mut c = Computer::from_str(input).unwrap();
    c.with_input(5).run().get_last_output()
}
