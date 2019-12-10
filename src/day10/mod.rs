use super::utils::points::Point;
use num::integer::gcd;
use std::cmp::{max, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[aoc(day10, part1)]
pub fn p1(input: &str) -> usize {
    let set: HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point(x as isize, y as isize)),
                _ => None,
            })
        })
        .collect();
    let mut best = 0;
    let mut map: HashMap<Point, HashSet<Point>> = HashMap::new();
    for p in set.iter() {
        let mut seen: HashSet<(isize, isize)> = HashSet::new();
        for other_p in set.iter() {
            if p == other_p {
                continue;
            }
            let y = other_p.1 - p.1;
            let x = other_p.0 - p.0;
            let g = gcd(x, y);
            if seen.insert((x / g, y / g)) {
                map.entry(p.clone()).or_default().insert(other_p.clone());
            }
        }

        best = max(best, seen.len());
    }
    let (k, v) = map.iter().max_by_key(|(k, v)| v.len()).unwrap();
    println!("Best is {:?}", k);
    v.len()
}

#[aoc(day10, part2)]
pub fn p2a(input: &str) -> isize {
    p2(input, Point(17, 22))
}
pub fn p2(input: &str, station: Point) -> isize {
    let set: HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point(x as isize, y as isize)),
                _ => None,
            })
        })
        .collect();
    let mut best = 0;
    let mut map: HashMap<(isize, isize), BinaryHeap<Reverse<(isize, Point)>>> = HashMap::new();
    for p in set.iter() {
        let o = *p - station;
        if o == Point(0, 0) {
            continue;
        }
        let y = p.1 - station.1;
        let x = p.0 - station.0;
        let g = gcd(x, y);
        map.entry((x / g, y / g))
            .or_insert(BinaryHeap::new())
            .push(Reverse((g, *p)));
    }

    let mut as_list: Vec<_> = map.iter().map(|(p, i)| (*p, i.clone())).collect();
    as_list.sort_by(|&(a, _), (b, _)| {
        let q = match (a.0 >= 0, a.1 >= 0) {
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
            (false, false) => 4,
        };
        let q2 = match (b.0 >= 0, b.1 >= 0) {
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
            (false, false) => 4,
        };
        let a1 = a.0 as f64 / a.1 as f64;
        let b1 = b.0 as f64 / b.1 as f64;
        (q.cmp(&q2)).then(b1.partial_cmp(&a1).unwrap_or(Ordering::Equal))
    });
    let mut c = 0;
    let mut list_ix = 0;
    let mut this = Point(0, 0);
    let mut non_empty_lists = as_list.len();
    while c < 200 && non_empty_lists > 0 {
        let heap = &mut as_list[list_ix].1;
        match heap.pop() {
            Some(Reverse((_, x))) => {
                this = x;
                println!("Zapped {} {:?}", c + 1, this);
                c += 1;
            }
            None => {
                non_empty_lists -= 1;
            }
        }
        list_ix = (list_ix + 1) % as_list.len();
    }
    this.0 * 100 + this.1
}

#[test]
pub fn tests() {
    assert_eq!(p1(".#..#\n.....\n#####\n....#\n...##"), 8);
    assert_eq!(p1("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n"),33);
    assert_eq!(p1("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###."),35);
    assert_eq!(p1(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..\n"),41);
}

#[test]
pub fn t2() {
    let e = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##";
    p2(&e, Point(8, 3));
}

/*           1111111
   01234567890123456
 0 .#....###24...#..
 1 ##...##.13#67..9#
 2 ##...#...5.8####.
 3 ..#.....X...###..
 4 ..#.#.....#....##

*/
