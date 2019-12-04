use super::utils::nums::{int_to_digits, is_sorted};
use itertools::Itertools;

pub fn check_groups(input: &[u8], check: fn(usize) -> bool) -> bool {
    input
        .iter()
        .group_by(|x| *x)
        .into_iter()
        .any(|x| check(x.1.count()))
}

pub fn find(input: &[usize], group_check: fn(usize) -> bool) -> usize {
    (input[0]..=input[1])
        .map(int_to_digits)
        .filter(|x| is_sorted(&x) && check_groups(&x, group_check))
        .count()
}

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Vec<usize> {
    input.split('-').map(|x| x.parse().unwrap()).collect()
}
#[aoc(day4, part1)]
pub fn p1(input: &[usize]) -> usize {
    find(input, |x| x > 1)
}
#[aoc(day4, part2)]
pub fn p2(input: &[usize]) -> usize {
    find(input, |x| x == 2)
}
