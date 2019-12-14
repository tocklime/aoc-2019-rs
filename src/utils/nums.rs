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

pub fn find_upper(func: &impl Fn(usize) -> usize, target: usize) -> usize {
    let mut upper = 1;
    loop {
        let output = func(upper);
        if output > target {
            return upper;
        }
        upper *= 2;
    }
}
pub fn bin_search(
    func: &impl Fn(usize) -> usize,
    target: usize,
    upper: usize,
    lower: usize,
) -> usize {
    let candidate = (upper + lower) / 2;
    if candidate == lower {
        return lower;
    }
    let val = func(candidate);
    if val > target {
        bin_search(func, target, candidate, lower)
    } else {
        bin_search(func, target, upper, candidate)
    }
}
pub fn unbounded_bin_search(func: impl Fn(usize) -> usize, target: usize) -> usize {
    let upper = find_upper(&func, target);
    bin_search(&func, target, upper, upper / 2)
}
