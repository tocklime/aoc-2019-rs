use super::utils::points::Point;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

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
    p1(input).0
}
pub fn p1(input: &Prob) -> (usize, Point) {
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
    p2(input, p1(input).1)
}
pub fn p2(input: &Prob, station: Point) -> isize {
    let mut map: HashMap<Point, BinaryHeap<Reverse<(isize, Point)>>> = HashMap::new();
    for &p in input.iter().filter(|&&x| x != station) {
        let o = p - station;
        map.entry(o.simplest_direction())
            .or_insert(BinaryHeap::new())
            .push(Reverse((o.size_squared(), p)));
    }
    let mut as_list: Vec<_> = map.iter().map(|(p, i)| (*p, i.clone())).collect();
    as_list.sort_by(|&(a, _), (b, _)| {
        (a.quadrant_clockwise().cmp(&b.quadrant_clockwise())).then(
            a.gradient()
                .partial_cmp(&b.gradient())
                .unwrap_or(Ordering::Equal),
        )
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
                //println!("Zapped {} {:?}", c + 1, this);
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
    assert_eq!(
        p1(&gen(".#..#\n.....\n#####\n....#\n...##")),
        (8, Point(3, 4))
    );
    assert_eq!(p1(&gen("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n")),(33,Point(5,8)));
    assert_eq!(p1(&gen("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.")),(35,Point(1,2)));
    assert_eq!(p1(&gen(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..\n")),(41, Point(6,3)));
}

#[test]
pub fn t2() {
    let e = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##";
    p2(&gen(&e), Point(8, 3));
}

/*           1111111
   01234567890123456
 0 .#....###24...#..
 1 ##...##.13#67..9#
 2 ##...#...5.8####.
 3 ..#.....X...###..
 4 ..#.#.....#....##

*/
