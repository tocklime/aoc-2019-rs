use std::collections::{HashMap,HashSet,BTreeSet};
use crate::utils::points::{Point,as_point_map};
use std::cmp::min;

pub fn search2(
    map: &HashMap<Point, char>,
    start: Point,
) -> HashMap<Point, (char, usize, BTreeSet<char>)> {
    let mut points = std::collections::VecDeque::new();
    points.push_back((start, 0, BTreeSet::new()));
    let mut min_dist_map: HashMap<Point, (char, usize, BTreeSet<char>)> = HashMap::new();
    let mut been: HashSet<Point> = HashSet::new();
    while !points.is_empty() {
        let (pos, count, keys) = points.pop_front().unwrap();
        been.insert(pos);
        pos.neighbours()
            .iter()
            .filter(|&p| !been.contains(p))
            .for_each(|&p2| {
                if let Some(here) = map.get(&p2) {
                    if here != &'#' && (here == &'.' || !min_dist_map.contains_key(&p2))
                    //&& (!here.is_uppercase() || keys.contains(&here.to_ascii_lowercase()))
                    {
                        let mut new_keys = keys.clone();
                        if here.is_lowercase() {
                            min_dist_map.insert(p2, (*here, count + 1, new_keys.clone()));
                        }
                        if here.is_alphabetic() {
                            new_keys.insert(*here);
                        }
                        points.push_back((p2, count + 1, new_keys));
                    }
                }
            });
    }
    min_dist_map
}
pub fn search(
    map: &HashMap<Point, char>,
    start: Point,
    keys: &BTreeSet<char>,
) -> HashMap<char, (usize, Point)> {
    let mut points = std::collections::VecDeque::new();
    points.push_back((start, 0));
    let mut min_dist_map = HashMap::new();
    let mut been: BTreeSet<Point> = BTreeSet::new();
    while !points.is_empty() {
        let (pos, count) = points.pop_front().unwrap();
        been.insert(pos);
        pos.neighbours()
            .iter()
            .filter(|&p| !been.contains(p))
            .for_each(|&p2| {
                if let Some(here) = map.get(&p2) {
                    if here != &'#'
                        && (here == &'.' || !min_dist_map.contains_key(&p2))
                        && (!here.is_uppercase() || keys.contains(&here.to_ascii_lowercase()))
                    {
                        if here.is_lowercase() && !keys.contains(&here) {
                            min_dist_map.insert(p2, (*here, (count + 1, p2)));
                        } else {
                            points.push_back((p2, count + 1));
                        }
                    }
                }
            });
    }
    min_dist_map
        .values()
        .filter(|(c, _)| c.is_lowercase() && !keys.contains(c))
        .cloned()
        .collect()
}
#[aoc(day18, part1)]
pub fn p1(input: &str) -> usize {
    let map = as_point_map(input);
    let at_sym = *map.iter().find(|(_, &v)| v == '@').expect("No @").0;
    solve(&map, &[at_sym])
}
#[aoc(day18, part2)]
pub fn p2(input: &str) -> usize {
    let mut map = as_point_map(input);
    let at_sym = *map.iter().find(|(_, &v)| v == '@').expect("No @").0;

    map.insert(at_sym, '#');
    map.insert(at_sym.up(), '#');
    map.insert(at_sym.down(), '#');
    map.insert(at_sym.left(), '#');
    map.insert(at_sym.right(), '#');
    let points = [
        at_sym.up().left(),
        at_sym.up().right(),
        at_sym.down().left(),
        at_sym.down().right(),
    ];
    solve(&map, &points)
}
pub fn solve(map: &HashMap<Point, char>, starts: &[Point]) -> usize {
    let mut known_bests: HashMap<(Vec<Point>, BTreeSet<char>), usize> = HashMap::new();
    known_bests.insert((starts.to_vec(), BTreeSet::new()), 0);
    let locations: Vec<Point> = map
        .iter()
        .filter_map(|(&k, &v)| if v.is_lowercase() { Some(k) } else { None })
        .chain(starts.iter().cloned())
        .collect();
    let info: HashMap<Point, HashMap<Point, (char, usize, BTreeSet<char>)>> = locations
        .iter()
        .map(|&ap| {
            let reachable = search2(&map, ap);
            (ap, reachable)
        })
        .collect();

    loop {
        let mut new_known_bests: HashMap<(Vec<Point>, BTreeSet<char>), usize> = HashMap::new();
        for ((poss, keys), v) in known_bests.iter() {
            for (ix, bot) in poss.iter().enumerate() {
                let available_keys = info[bot].iter().filter(|(_, (c, _, hs))| {
                    !keys.contains(c) && hs.iter().all(|&i| keys.contains(&i.to_ascii_lowercase()))
                });
                for (new_p, (c, d, _)) in available_keys {
                    let mut new_keys = keys.clone();
                    new_keys.insert(*c);
                    let mut new_ps = poss.clone();
                    new_ps[ix] = *new_p;
                    new_known_bests
                        .entry((new_ps, new_keys))
                        .and_modify(|e| *e = min(*e, d + v))
                        .or_insert(d + v);
                }
            }
        }
        if new_known_bests.is_empty() {
            break;
        }
        known_bests = new_known_bests;
    }
    *known_bests.values().min().expect("No answers?")
}

#[cfg(test)]
mod test {
    //                  012345678901234567890123
    const MAZ0: &str = "########################\n\
                        #f.D.E.e.C.b.A.@.a.B.c.#\n\
                        ######################.#\n\
                        #d.....................#\n\
                        ########################";

    const MAZ1: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
";
    const MAZ2: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
";

    #[test]
    pub fn d18p1tests() {
        assert_eq!(super::p1(&MAZ0), 86);
        assert_eq!(super::p1(&MAZ1), 132);
        assert_eq!(super::p1(&MAZ2), 136);
    }
}
