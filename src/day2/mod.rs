use super::comp::Computer;

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn run_with_args(c: &mut Computer, noun: usize, verb: usize) -> usize {
    c.abs_store(1, noun);
    c.abs_store(2, verb);
    c.run().abs_load(0)
}

#[aoc(day2, part1)]
pub fn p1(input: &[usize]) -> usize {
    run_with_args(&mut Computer::new(input), 12, 2)
}

#[aoc(day2, part2)]
pub fn p2(input: &[usize]) -> usize {
    let mut c = Computer::new(input);
    for n in 0..100 {
        for v in 0..100 {
            c.reset();
            if run_with_args(&mut c, n, v) == 19690720 {
                return 100 * n + v;
            }
        }
    }
    panic!("Cannot find solution for part 2");
}
