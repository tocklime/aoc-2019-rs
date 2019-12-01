use itertools::unfold;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().cloned().map(part1_calc).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input.iter().cloned().map(part2_calc).sum()
}

pub fn part1_calc(x: i32) -> i32 {
    (x / 3) - 2
}

pub fn part2_calc(x: i32) -> i32 {
    unfold(x, |last_mass| match part1_calc(*last_mass) {
        a if a <= 0 => None,
        a => {
            *last_mass = a;
            Some(a)
        }
    })
    .sum()
}

#[test]
pub fn part1_examples() {
    assert_eq!(part1_calc(12), 2);
    assert_eq!(part1_calc(14), 2);
    assert_eq!(part1_calc(1969), 654);
    assert_eq!(part1_calc(100756), 33583);
}

#[test]
pub fn part2_examples() {
    assert_eq!(part2_calc(14), 2);
    assert_eq!(part2_calc(1969), 966);
    assert_eq!(part2_calc(100756), 50346);
}
