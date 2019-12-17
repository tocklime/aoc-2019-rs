use crate::utils::prelude::*;

#[aoc(day17, part1)]
pub fn p1(input: &str) -> isize {
    let mut c: Computer = input.parse().unwrap();
    c.run();
    let output = c.take_output();
    let as_chars: String = output.iter().map(|&x| (x as u8) as char).collect();
    let g: HashMap<Point, char> = as_chars
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point(x as isize, y as isize), c))
        })
        .collect();
    g.iter()
        .filter(|(_, &c)| c == '#')
        .filter(|(p, _)| {
            p.neighbours()
                .iter()
                .filter(|pn| g.get(pn) == Some(&'#'))
                .count()
                == 4
        })
        .map(|(p, _)| p.0 * p.1)
        .sum()
}

#[aoc(day17, part2)]
pub fn p2(input: &str) -> i32 {
    let mut c: Computer = input.parse().unwrap();
    c.abs_store(0, 2);
    let icode: &str = "
A,B,A,C,A,B,C,A,B,C
R,8,R,10,R,10
R,4,R,8,R,10,R,12
R,12,R,4,L,12,L,12
n
";
    c.give_input(icode.trim_start().chars().map(|x| x as i32).collect());
    c.run();
    c.get_last_output()
}
