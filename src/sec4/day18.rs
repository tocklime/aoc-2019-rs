use crate::utils::points::as_point_map;
use crate::utils::prelude::*;
use std::cmp::min;
const WALL: char = '#';

//                  key           key    dist   blocking doors
type Info = HashMap<char, HashMap<char, (usize, HashSet<char>)>>;

pub fn make_graph(map: &HashMap<Point, char>) -> HashMap<char, HashSet<char>> {
    // list of doors each key is behind.
    let start = map.iter().find(|(_, &c)| c == '@').unwrap().0;
    let mut points: Vec<(Point, HashSet<char>)> = Vec::new();
    let mut ans = HashMap::new();
    points.push((*start, HashSet::new()));
    let mut been: HashSet<Point> = HashSet::new();
    while !points.is_empty() {
        let (pos, blockingdoors) = points.pop().unwrap();
        been.insert(pos);
        pos.neighbours()
            .iter()
            .filter(|&p| !been.contains(p))
            .for_each(|&p2| {
                if let Some(here) = map.get(&p2) {
                    if here == &WALL {
                        return;
                    }
                    let mut doors = blockingdoors.clone();
                    if here.is_lowercase() {
                        //got key
                        ans.insert(*here, doors.clone());
                    } else if here.is_uppercase() {
                        //found door.
                        doors.insert(here.to_ascii_lowercase());
                    } else {
                        //just walking.
                    }
                    points.push((p2, doors));
                }
            });
    }
    ans
}

pub fn all_dists(map: &HashMap<Point, char>) -> HashMap<(char, char), usize> {
    let all_things: Vec<_> = map
        .iter()
        .filter(|(k, x)| x.is_alphabetic() || *x == &'@')
        .collect();
    let mut ans: HashMap<(char, char), usize> = HashMap::new();
    for ix in 0..all_things.len() {
        for ix2 in ix + 1..all_things.len() {
            let (&xp, &x) = all_things[ix];
            let (&yp, &y) = all_things[ix2];
            let d = bfs_dist(&map, xp, y);
            ans.insert((x, y), d);
            ans.insert((y, x), d);
        }
    }
    ans
}

pub fn bfs_dist(map: &HashMap<Point, char>, start: Point, end: char) -> usize {
    let mut points = Vec::new();
    points.push((start, 0));
    let mut been: HashSet<Point> = HashSet::new();
    let mut ans = std::usize::MAX;
    while !points.is_empty() {
        let (pos, count) = points.pop().unwrap();
        been.insert(pos);
        pos.neighbours()
            .iter()
            .filter(|&p| !been.contains(p))
            .for_each(|&p2| {
                if let Some(here) = map.get(&p2) {
                    if here != &WALL {
                        if here == &end {
                            ans = count + 1;
                        } else {
                            points.push((p2, count + 1));
                        }
                    }
                }
            });
    }
    ans
}
pub fn bfs(
    map: &HashMap<Point, char>,
    start: Point,
    keys: &BTreeSet<char>,
) -> HashMap<char, (usize, Point)> {
    let mut points = std::collections::VecDeque::new();
    points.push_back((start, 0));
    let mut min_dist_map = HashMap::new();
    let mut been: HashSet<Point> = HashSet::new();
    while !points.is_empty() {
        let (pos, count) = points.pop_front().unwrap();
        been.insert(pos);
        pos.neighbours()
            .iter()
            .filter(|&p| !been.contains(p))
            .for_each(|&p2| {
                if let Some(here) = map.get(&p2) {
                    if here != &WALL
                        && (here == &'.' || !min_dist_map.contains_key(&p2))
                        && (!here.is_uppercase() || keys.contains(&here.to_ascii_lowercase()))
                    {
                        if here.is_lowercase() && !keys.contains(&here) {
                            min_dist_map.insert(p2, (*here, (count + 1, p2)));
                        } else {
                            //println!("Try later: {:?}", p2);
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
use std::collections::BTreeSet;
#[aoc(day18, part1)]
pub fn p1(input: &str) -> usize {
    let map = as_point_map(input);
    let locations: HashMap<char, Point> = map.iter().map(|(k, v)| (*v, *k)).collect();
    let g = make_graph(&map);
    let dists = all_dists(&map);
    println!("G: {:?}", g);
    println!("D: {:?}", dists);
    let key_count = map.iter().filter(|(_, c)| c.is_lowercase()).count();
    let mut saves = Vec::new();
    let mut known_bests: HashMap<(Point, BTreeSet<char>), usize> = HashMap::new();
    known_bests.insert((locations[&'@'], BTreeSet::new()), 0);
    for i in 0..key_count {
        let mut new_known_bests = HashMap::new();
        for ((pos, keys), v) in known_bests.iter() {
            let available_keys = bfs(&map, *pos, &keys);
            for (new_k, (d, p)) in available_keys.iter() {
                let mut new_keys = keys.clone();
                new_keys.insert(*new_k);
                new_known_bests
                    .entry((*p, new_keys))
                    .and_modify(|e| *e = min(*e, d + v))
                    .or_insert(d + v);
            }
        }
        println!("{:?}", new_known_bests);
        known_bests = new_known_bests;
    }
    return *known_bests.values().min().expect("No answers?");
    saves.push(('@', 0, HashSet::new()));
    let mut anss: Vec<usize> = Vec::new();
    while !saves.is_empty() {
        let (pos, dist, owned_keys) = saves.pop().unwrap();
        let available_keys: Vec<char> = g
            .iter()
            .filter(|(k, req)| req.is_subset(&owned_keys) && !owned_keys.contains(k))
            .map(|x| x.0)
            .cloned()
            .collect();
        for &k in &available_keys {
            let mut c = owned_keys.clone();
            c.insert(k);
            let new_dist = dists
                .get(&(pos, k))
                .unwrap_or_else(|| panic!("No dist {} -> {}", pos, k));
            saves.push((k, dist + new_dist, c));
        }
        if available_keys.is_empty() {
            anss.push(dist);
        }
    }
    *anss.iter().min().unwrap()

    //find a thing we could get

    /*     let me = *map.iter().find(|(_, &b)| b == '@').unwrap().0;
    //let key_count = map.iter().filter(|(_, c)| c.is_lowercase()).count();
    //println!("There are {} keys to get", key_count);
    let mut saves = VecDeque::new();
    saves.push_front((0, me, HashSet::new()));
    let mut best_ans = std::usize::MAX;
    let mut lim = 0;
    while !saves.is_empty() && lim < 1000 {
        lim += 1;
        let (dist, point, keys) = saves.pop_back().unwrap();
        let a = bfs(&map, point, &keys);
        /*       println!(
            "At {:?}, with {:?}, have a choice of {:?}",
            point,
            keys,
            a.keys()
        ); */
        for (c, (d, p)) in a.iter() {
            let mut k = keys.clone();
            k.insert(*c);
            saves.push_back((dist + d, *p, k));
        }
        if a.is_empty() {
            best_ans = min(best_ans, dist);
        }
    }
    best_ans */
}

#[cfg(test)]
mod test {

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
