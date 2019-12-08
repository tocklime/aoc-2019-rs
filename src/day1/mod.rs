use itertools::{iterate, unfold};

pub fn rocket_fn(x: i32) -> i32 {
    (x / 3) - 2
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().cloned().map(rocket_fn).sum()
}

#[aoc(day1, part2, unfold)]
pub fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .flat_map(|x| {
            unfold(*x, |last_mass| match rocket_fn(*last_mass) {
                a if a <= 0 => None,
                a => {
                    *last_mass = a;
                    Some(a)
                }
            })
        })
        .sum()
}
#[aoc(day1, part2, iterate)]
pub fn part2_2(input: &[i32]) -> i32 {
    input
        .iter()
        .cloned()
        .flat_map(|x| iterate(x, |&y| rocket_fn(y)).skip(1).take_while(|&x| x > 0))
        .sum()
}

#[test]
pub fn part1_examples() {
    assert_eq!(part1(&[12]), 2);
    assert_eq!(part1(&[14]), 2);
    assert_eq!(part1(&[1969]), 654);
    assert_eq!(part1(&[100756]), 33583);
}

#[test]
pub fn part2_examples() {
    assert_eq!(part2(&[14]), 2);
    assert_eq!(part2(&[1969]), 966);
    assert_eq!(part2(&[100756]), 50346);
    assert_eq!(part2_2(&[14]), 2);
    assert_eq!(part2_2(&[1969]), 966);
    assert_eq!(part2_2(&[100756]), 50346);
}
