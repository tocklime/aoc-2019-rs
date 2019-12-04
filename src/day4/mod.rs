use super::utils::nums::int_to_digits;
use itertools::Itertools;
use rayon::prelude::*;

pub fn check_groups(input: &[u8], check: fn(usize) -> bool) -> bool {
    let groups = input.iter().group_by(|x| *x);
    let mut last_key: Option<u8> = None;
    let mut saw_any_match = false;
    for (&k, g) in groups.into_iter() {
        if last_key.map_or(false, |l| l > k) {
            return false; //key decreased!
        }
        last_key = Some(k);
        saw_any_match |= check(g.count());
    }
    saw_any_match
}

pub fn find(input: &[usize], group_check: fn(usize) -> bool) -> usize {
    (input[0]..=input[1])
        .into_par_iter()
        .filter(|&x| check_groups(&int_to_digits(x), group_check))
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
