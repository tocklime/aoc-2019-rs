use crate::utils::prelude::*;

pub fn go(input: &str, stringscript: &str) -> Option<i32> {
    let mut c: Computer = input.parse().unwrap();
    c.give_input(
        stringscript
            .trim_start()
            .chars()
            .map(|x| x as i32)
            .collect(),
    );
    c.run();
    let output = c.take_output();
    output.iter().find(|&&x| x > 255).cloned()
}
#[aoc(day21, part1)]
pub fn p1(input: &str) -> i32 {
    //(a+b+c)*D
    go(
        input,
        "\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
",
    )
    .unwrap()
}

#[aoc(day21, part2)]
pub fn p2a(input: &str) -> i32 {
    //Jump if there's a hole and we can either step or jump after.
    //(a + b + c) & D & (E + H)
    go(
        input,
        "\
NOT A T 
NOT B J
OR T J
NOT C T
OR T J
AND D J
AND J T
AND E T
OR H T
AND T J
RUN
",
    )
    .unwrap_or(0)
}
