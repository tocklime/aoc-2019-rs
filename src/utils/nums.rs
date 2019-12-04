pub fn int_to_digits(i: usize) -> Vec<u8> {
    format!("{}", i)
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

pub fn is_sorted(i: &[u8]) -> bool {
    i.iter().zip(i.iter().skip(1)).all(|(a, b)| a <= b)
}
