use super::utils::points::{Point, PolarCoord};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::f64::consts::FRAC_PI_2;

// map of point to map of direction to list of (distance,point) tuples (in ascending order of distance)
type AsteroidSet = HashSet<Point>;
type Analysed = HashMap<Point, HashMap<Point, BinaryHeap<Reverse<(isize, Point)>>>>;
#[aoc_generator(day10)]
pub fn gen(input: &str) -> AsteroidSet {
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
fn analyse(input: &AsteroidSet) -> Analysed {
    input
        .iter()
        .map(|&p| {
            (
                p,
                input
                    .iter()
                    .filter(|&&x| x != p)
                    .fold(HashMap::new(), |mut hm, &other_p| {
                        let rel = p - other_p;
                        hm.entry(rel.simplest_direction())
                            .or_insert(BinaryHeap::new())
                            .push(Reverse((rel.size_squared(), other_p)));
                        hm
                    }),
            )
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn p1a(input: &AsteroidSet) -> usize {
    get_best_station(input).0
}
pub fn get_best_station(input: &AsteroidSet) -> (usize, Point) {
    let a = analyse(input);
    let (&k, v) = a.iter().max_by_key(|x| x.1.len()).unwrap();
    (v.len(), k)
}

#[aoc(day10, part2)]
pub fn p2a(input: &AsteroidSet) -> isize {
    p2(input, get_best_station(input).1, 200)
}

pub fn p2(input: &AsteroidSet, station: Point, nth: usize) -> isize {
    /*let mut map: HashMap<Point, BinaryHeap<Reverse<(isize, Point)>>> = HashMap::new();
     for &p in input.iter().filter(|&&x| x != station) {
        let o = p - station;
        map.entry(o.simplest_direction())
            .or_insert(BinaryHeap::new())
            .push(Reverse((o.size_squared(), p)));
    }
    as_list.sort_by(|&(a, _), &(b, _)| {
        let apc = PolarCoord::from_point(a).rotate(FRAC_PI_2);
        let bpc = PolarCoord::from_point(b).rotate(FRAC_PI_2);
        bpc.theta.partial_cmp(&apc.theta).unwrap()
    }); */
    println!("{:?}", input);
    let mut analysis = analyse(input);
    let st = analysis.get_mut(&station).unwrap();
    let mut dir_list: Vec<_> = st.keys().cloned().collect();
    dir_list.sort_by(|&a, &b| {
        let apc = PolarCoord::from_point(a); //.rotate(FRAC_PI_2);
        let bpc = PolarCoord::from_point(b); //.rotate(FRAC_PI_2);
        bpc.theta.partial_cmp(&apc.theta).unwrap()
    });
    let mut list_ix = 0;
    let mut order = Vec::new();
    while !dir_list.is_empty() {
        list_ix = list_ix % dir_list.len();
        let heap = &mut st.get_mut(&dir_list[list_ix]).unwrap();
        match heap.pop() {
            Some(Reverse((_, x))) => {
                order.push(x);
                list_ix += 1;
            }
            None => {
                dir_list.remove(list_ix);
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
    let e = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##";
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
