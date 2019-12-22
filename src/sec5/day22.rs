#[aoc(day22, part1)]
pub fn p1(input: &str) -> usize {
    let card_count = 10007_u32;
    let (offset, increment) = handle_deck(input, card_count as i128);
    let mut deck = vec![0; card_count as usize];
    let mut cur_val = offset;
    for i in 0..card_count {
        deck[i as usize] = cur_val % 10007;
        cur_val += increment
    }
    deck.iter().enumerate().find(|x| x.1 == &2019).unwrap().0
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
fn mod_inv(base: i128, modulus: i128) -> i128 {
    mod_pow(base, modulus - 2, modulus)
}
pub fn handle_deck(input: &str, deck_size: i128) -> (i128, i128) {
    let mut offset = 0_i128;
    let mut increment = 1_i128;
    for l in input.trim().lines() {
        //println!("Deck now {:?} {:?}   {}", offset, increment, l);
        if l.trim().starts_with("deal into new stack") {
            increment *= -1_i128;
            offset += increment;
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
            increment *= mod_inv(n, deck_size);
        } else {
            panic!("Unknown instr: {}", l);
        }
        increment %= deck_size;
        offset %= deck_size;
    }
    (offset, increment)
}
#[aoc(day22, part2)]
pub fn p2(input: &str) -> i128 {
    let p1rev = gop2(input, 10007, 1, 6526);
    assert_eq!(p1rev, 2019);
    gop2(input, 119315717514047_i128, 101741582076661_i128, 2020)
}
pub fn gop2(input: &str, deck_size: i128, shuffle_count: i128, card: i128) -> i128 {
    let (offset, increment) = handle_deck(input, deck_size);
    let final_increment = mod_pow(increment, shuffle_count, deck_size);
    let num = deck_size + final_increment - 1;
    let denom = mod_inv(increment - 1, deck_size);
    let final_offset = (offset * num % deck_size) * denom % deck_size;
    (final_offset + final_increment * card) % deck_size
}
