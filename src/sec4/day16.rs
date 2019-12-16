#[aoc_generator(day16)]
pub fn gen(input: &str) -> Vec<i32> {
    input.bytes().map(|x| (x - b'0') as i32).collect()
}
#[aoc(day16, part1)]
pub fn p1(input: &[i32]) -> i32 {
    let mut x = input.to_vec();
    (0..100).for_each(|_| x = fft(&x));
    list_to_int(&x[..8])
}
pub fn list_to_int(l: &[i32]) -> i32 {
    l.iter().fold(0, |n, &d| 10 * n + d)
}
#[aoc(day16, part2)]
pub fn p2(input: &[i32]) -> i32 {
    let offset: usize = list_to_int(&input[..7]) as usize;
    let mut input: Vec<_> = input
        .into_iter()
        .cycle()
        .take(input.len() * 10000)
        .skip(offset)
        .cloned()
        .collect();
    for _ in 0..100 {
        let mut sum = input.iter().sum::<i32>();
        for i in &mut input {
            let tmp = *i;
            *i = sum % 10;
            sum -= tmp;
        }
    }
    list_to_int(&input[..8])
}

pub fn fft(input: &[i32]) -> Vec<i32> {
    (0..input.len())
        .map(|ix| {
            let pos = (ix..)
                .step_by(4 * (ix + 1))
                .flat_map(|x| (x..x + 1 + ix))
                .take_while(|&x| x < input.len())
                .map(|i| input[i])
                .sum::<i32>();
            let neg = (3 * ix + 2..)
                .step_by(4 * (ix + 1))
                .flat_map(|x| (x..x + 1 + ix))
                .take_while(|&x| x < input.len())
                .map(|i| input[i])
                .sum::<i32>();
            (pos - neg).abs() % 10
        })
        .collect()
}
#[test]
pub fn day16p1test() {
    assert_eq!(fft(&[1, 2, 3, 4, 5, 6, 7, 8]), [4, 8, 2, 2, 6, 1, 5, 8]);
    assert_eq!(fft(&[4, 8, 2, 2, 6, 1, 5, 8]), [3, 4, 0, 4, 0, 4, 3, 8]);
    assert_eq!(p1(&gen(&"80871224585914546619083218645595")), 24176176);
    assert_eq!(p1(&gen(&"19617804207202209144916044189917")), 73745418);
    assert_eq!(p1(&gen(&"69317163492948606335995924319873")), 52432133);
}

#[test]
pub fn day16p2test() {
    assert_eq!(p2(&gen(&"03036732577212944063491565474664")), 84462026);
    assert_eq!(p2(&gen(&"02935109699940807407585447034323")), 78725270);
    assert_eq!(p2(&gen(&"03081770884921959731165446850517")), 53553731);
}
