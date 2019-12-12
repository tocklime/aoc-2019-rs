use std::ops::AddAssign;
pub fn int_to_digits(i: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(10);
    let mut r = i;
    while r > 0 {
        v.push(r % 10);
        r /= 10;
    }
    v.reverse();
    v
}

pub fn is_sorted(i: &[u8]) -> bool {
    i.iter().zip(i.iter().skip(1)).all(|(a, b)| a <= b)
}


pub fn de_prefixsum<T: AddAssign + Default + Copy>(input: &[T]) -> Vec<T> {
    let mut total: T = Default::default();
    let mut ans = Vec::with_capacity(input.len());
    for i in input {
        total += *i;
        ans.push(total);
    }
    ans
}
