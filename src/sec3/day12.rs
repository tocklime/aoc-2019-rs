use crate::utils::iter::all_ix_pairs;
use num::integer::lcm;
use regex::Regex;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Moon {
    pos: Vec<i64>,
    vel: Vec<i64>,
}

impl FromStr for Moon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        if let Some(m) = re.captures(s) {
            Ok(Moon {
                pos: vec![
                    m[1].parse().unwrap(),
                    m[2].parse().unwrap(),
                    m[3].parse().unwrap(),
                ],
                vel: vec![0, 0, 0],
            })
        } else {
            Err("No regex match".to_string())
        }
    }
}
impl Moon {
    fn step(&mut self) {
        for (i, p) in self.pos.iter_mut().enumerate() {
            *p += self.vel[i];
        }
    }
    fn energy(&self) -> i64 {
        let pot: i64 = self.pos.iter().map(|d| d.abs()).sum();
        let kin: i64 = self.vel.iter().map(|d| d.abs()).sum();
        pot * kin
    }
}

#[aoc_generator(day12)]
pub fn gen(input: &str) -> Vec<Moon> {
    input.lines().map(|l| l.parse::<Moon>().unwrap()).collect()
}

fn do_gravity(moons: &mut [Moon], dimension: usize) {
    all_ix_pairs(moons.len()).for_each(|(m1ix, m2ix)| {
        let s = (moons[m1ix].pos[dimension] - moons[m2ix].pos[dimension]).signum();
        moons.get_mut(m1ix).unwrap().vel[dimension] -= s;
        moons.get_mut(m2ix).unwrap().vel[dimension] += s;
    })
}

#[aoc(day12, part1)]
pub fn p1(input: &[Moon]) -> i64 {
    let mut moons = input.to_vec();
    for _ in 0..1000 {
        (0..3).for_each(|d| do_gravity(&mut moons, d));
        moons.iter_mut().for_each(Moon::step);
    }
    moons.iter().map(Moon::energy).sum()
}

#[aoc(day12, part2)]
pub fn p2(input: &[Moon]) -> usize {
    let mut moons = input.to_vec();
    let periods = (0..3)
        .map(|d| {
            (1..)
                .find(|_| {
                    do_gravity(&mut moons, d);
                    for m in moons.iter_mut() {
                        m.pos[d] += m.vel[d];
                    }
                    return moons == input;
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    periods.iter().cloned().fold(1, lcm)
}
