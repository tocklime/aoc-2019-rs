use super::comp::Computer;

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}
#[aoc(day9, part1)]
pub fn p1(input: &[isize]) -> isize {
    let mut c = Computer::new(input);
    c.with_input(1);
    c.run();
    c.get_output()
}
#[aoc(day9, part2)]
pub fn p2(input: &[isize]) -> isize {
    let mut c = Computer::new(input);
    c.with_input(2);
    c.run();
    c.get_output()
}
#[test]
pub fn p1tests() {
    let e0 = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let e1 = "1102,34915192,34915192,7,4,7,99,0";
    let e2 = "104,1125899906842624,99";
    assert_eq!(p1(&gen(e0)), 99);
    assert_eq!(p1(&gen(e1)), 1219070632396864);
    assert_eq!(p1(&gen(e2)), 1125899906842624);
}
