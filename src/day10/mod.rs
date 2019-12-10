use super::utils::points::{Point, PolarCoord};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::f64::consts::FRAC_PI_2;

type Prob = HashSet<Point>;
#[aoc_generator(day10)]
pub fn gen(input: &str) -> Prob {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point(x as isize, y as isize)),
                _ => None,
            })
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn p1a(input: &Prob) -> usize {
    get_best_station(input).0
}
pub fn get_best_station(input: &Prob) -> (usize, Point) {
    let mut map: HashMap<Point, HashSet<Point>> = HashMap::new();
    for &p in input.iter() {
        let mut seen: HashSet<Point> = HashSet::new();
        for &other_p in input.iter().filter(|&&x| x != p) {
            if seen.insert((other_p - p).simplest_direction()) {
                map.entry(p).or_default().insert(other_p);
            }
        }
    }
    let (&k, v) = map.iter().max_by_key(|(_, v)| v.len()).unwrap();
    (v.len(), k)
}

#[aoc(day10, part2)]
pub fn p2a(input: &Prob) -> isize {
    p2(input, get_best_station(input).1, 200)
}

pub fn p2(input: &Prob, station: Point, nth: usize) -> isize {
    let mut map: HashMap<Point, BinaryHeap<Reverse<(isize, Point)>>> = HashMap::new();
    for &p in input.iter().filter(|&&x| x != station) {
        let o = p - station;
        map.entry(o.simplest_direction())
            .or_insert(BinaryHeap::new())
            .push(Reverse((o.size_squared(), p)));
    }
    let mut as_list: Vec<_> = map.iter().map(|(p, i)| (*p, i.clone())).collect();
    as_list.sort_by(|&(a, _), &(b, _)| {
        let apc = PolarCoord::from_point(a).rotate(FRAC_PI_2);
        let bpc = PolarCoord::from_point(b).rotate(FRAC_PI_2);
        bpc.theta.partial_cmp(&apc.theta).unwrap()
    });
    let mut list_ix = 0;
    let mut order = Vec::new();
    while !as_list.is_empty() {
        list_ix = list_ix % as_list.len();
        let heap = &mut as_list[list_ix].1;
        match heap.pop() {
            Some(Reverse((_, x))) => {
                order.push(x);
                list_ix += 1;
            }
            None => {
                as_list.remove(list_ix);
            }
        }
    }
    let x = order[nth - 1];
    x.0 * 100 + x.1
}

#[test]
pub fn tests() {
    assert_eq!(
        get_best_station(&gen(".#..#\n.....\n#####\n....#\n...##")),
        (8, Point(3, 4))
    );
    assert_eq!(get_best_station(&gen("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n")),(33,Point(5,8)));
    assert_eq!(get_best_station(&gen("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.")),(35,Point(1,2)));
    assert_eq!(get_best_station(&gen(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..\n")),(41, Point(6,3)));
}

#[test]
pub fn t2() {
    let e = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##";
    assert_eq!(p2(&gen(&e), Point(8, 3), 1), 801);
}

/*           1111111
   01234567890123456
 0 .#....###24...#..
 1 ##...##.13#67..9#
 2 ##...#...5.8####.
 3 ..#.....X...###..
 4 ..#.#.....#....##

*/
