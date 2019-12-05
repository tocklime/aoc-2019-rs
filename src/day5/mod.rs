use super::comp::Computer;

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn p1(input: &[isize]) -> isize {
    let mut c = Computer::new(input);
    c.with_input(1);
    c.run();
    c.get_output()
}

#[aoc(day5, part2)]
pub fn p2(input: &[isize]) -> isize {
    let mut c = Computer::new(input);
    c.with_input(5);
    c.run();
    c.get_output()
}
