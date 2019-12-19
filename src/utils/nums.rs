use num::Integer;
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

pub fn find_upper<T: Integer + Copy>(func: &impl Fn(T) -> T, target: T) -> T {
    let mut upper = T::one();
    loop {
        let output = func(upper);
        if output >= target {
            return upper;
        }
        upper = upper + upper;
    }
}
pub fn bin_search<T: Integer + Copy>(func: &impl Fn(T) -> T, target: T, upper: T, lower: T) -> T {
    let candidate = (upper + lower) / (T::one() + T::one());
    if candidate == lower {
        return lower;
    }
    let val = func(candidate);
    if val >= target {
        bin_search(func, target, candidate, lower)
    } else {
        bin_search(func, target, upper, candidate)
    }
}
pub fn unbounded_bin_search<T: Integer + Copy>(func: impl Fn(T) -> T, target: T) -> T {
    let upper = find_upper(&func, target);
    bin_search(&func, target, upper, upper / (T::one() + T::one()))
}
