use crate::utils::prelude::HashSet;
use num::bigint::BigInt;
use num::Zero;
use std::convert::TryInto;
use std::ops::{AddAssign, Mul, MulAssign};

pub fn track(input: &str, card_count: u32, card: u32) -> u32 {
    let mut known_pos = card.clone();
    for l in input.trim().lines() {
        if l.trim().starts_with("deal into new stack") {
            known_pos = card_count - known_pos - 1;
        } else if l.trim().starts_with("cut") {
            let n = l
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .expect("int for cut");
            known_pos = ((known_pos as i32) - n)
                .rem_euclid(card_count as i32)
                .try_into()
                .unwrap();
        } else if l.trim().starts_with("deal with increment") {
            let n = l
                .split(' ')
                .nth(3)
                .unwrap()
                .parse::<u32>()
                .expect("int for deal");
            known_pos = (known_pos * n).rem_euclid(card_count);
        } else {
            panic!("Unknown instr: {}", l);
        }
    }
    known_pos
}
#[aoc(day22, part1)]
pub fn p1(input: &str) -> usize {
    let card_count = 10007_u32;
    let (offset, increment) = handle_deck(input, card_count as i128);
    let mut deck = vec![0; card_count as usize];
    let mut curVal = offset.clone();
    for i in 0..card_count {
        deck[i as usize] = curVal.value;
        curVal += increment.value
    }
    deck.iter().enumerate().find(|x| x.1 == &2019).unwrap().0
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModInt {
    value: i128,
    modulus: i128,
}
fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
impl ModInt {
    pub fn new(val: i128, mod_val: i128) -> Self {
        ModInt {
            value: val,
            modulus: mod_val,
        }
    }
    pub fn inv_mod(&mut self) -> i128 {
        let x = mod_pow(self.value, self.modulus - 2, self.modulus);
        println!("{}^{}-1 = {}, self.modulus", self.value, self.modulus, x);
        assert_eq!(x * self.modulus, 1);
        x
    }
}
impl Mul<i128> for ModInt {
    type Output = i128;
    fn mul(self, rhs: i128) -> Self::Output {
        let n: i128 = (self.value * rhs).rem_euclid(self.modulus);
        n
    }
}
impl MulAssign<i128> for ModInt {
    fn mul_assign(&mut self, rhs: i128) {
        let n: i128 = (self.value * rhs).rem_euclid(self.modulus);
        self.value = n
    }
}
impl AddAssign<i128> for ModInt {
    fn add_assign(&mut self, rhs: i128) {
        let n: i128 = (self.value + rhs).rem_euclid(self.modulus);
        self.value = n;
    }
}
pub fn handle_deck(input: &str, deck_size: i128) -> (ModInt, ModInt) {
    let mut offset = ModInt::new(0, deck_size);
    let mut increment = ModInt::new(1, deck_size);
    for l in input.trim().lines() {
        //println!("Deck now {:?} {:?}   {}", offset, increment, l);
        if l.trim().starts_with("deal into new stack") {
            increment *= -1_i128;
            offset += increment.value;
        } else if l.trim().starts_with("cut") {
            let n = l
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<i128>()
                .expect("int for cut");
            offset += increment * n;
        } else if l.trim().starts_with("deal with increment") {
            let n = l
                .split(' ')
                .nth(3)
                .unwrap()
                .parse::<i128>()
                .expect("int for deal");
            let inv = mod_pow(n, increment.modulus - 2, increment.modulus);
            increment *= inv;
        } else {
            panic!("Unknown instr: {}", l);
        }
    }
    (offset, increment)
}
#[aoc(day22, part2)]
pub fn p2(input: &str) -> i128 {
    let deck_size = 119315717514047_i128;
    let (offset, increment) = handle_deck(input, deck_size);
    println!("Deck now {:?} {:?}", offset, increment);
    let shuffle_count = 101741582076661_i128;
    let final_increment = mod_pow(increment.value, shuffle_count, deck_size);
    let r_to_n = mod_pow(increment.value, shuffle_count, deck_size);
    let r = increment.value;
    let final_offset = (offset.value * (deck_size - r_to_n)
        / mod_pow(increment.value - 1, deck_size - 2, deck_size))
        % deck_size;
    let a = final_increment;
    let b = final_offset;
    let x = mod_pow(a, shuffle_count, deck_size) * 2020
        + b * (mod_pow(a, shuffle_count, deck_size) + deck_size - 1)
            * (mod_pow(a - 1, deck_size - 2, deck_size));
    println!("{}", x % deck_size);
    println!("{} {}", final_increment, final_offset);
    (final_offset + final_increment * 2020) % deck_size
}
//43438952059674 - too low.
//115907756411024 - too high.
//112406479601211 - too high.
//61564889512447465 - too high...
//117294992713260 - too high...
//19206856320777 -x
//81965313299241 - incorrect.
/*
pub fn card_pos(deck: &Vec<i128>, n: i128) -> usize {
    deck.iter()
        .enumerate()
        .find(|x| x.1 == &n)
        .expect("card not found")
        .0
}
pub fn mymod(a: I, b: &I) -> I {
    let x = a % b;
    if x < I::zero() {
        x + b
    } else {
        x
    }
}


#[cfg(test)]
pub fn check_deck(input: &str, correct: &[i128]) {
    for (i, e) in correct.iter().enumerate() {
        assert_eq!(track(input, &I::from(10), &I::from(*e)), I::from(i));
    }
}
#[test]
pub fn d22p1() {
    let e1 = "
    deal with increment 7
    deal into new stack
    deal into new stack";
    check_deck(e1, &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

    let e2 = "\
cut 6
deal with increment 7
deal into new stack";
    check_deck(&e2, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    let e3 = "\
deal with increment 7
deal with increment 9
cut -2";
    check_deck(&e3, &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

    // 8 9 0 1 2 3 4 5 6 7
    // 8
    let e4 = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
    check_deck(&e4, &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
}*/
