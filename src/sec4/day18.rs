use crate::utils::points::as_point_map;
use crate::utils::prelude::*;
use std::cmp::min;

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
                    if here != &'#'
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
    let at_sym = *map.iter().find(|(_,&v)| v == '@').expect("No @").0;
    solve(&map,&[at_sym])
}
#[aoc(day18, part2)]
pub fn p2(input: &str) -> usize {
    let mut map = as_point_map(input);
    let at_sym = *map.iter().find(|(_,&v)| v == '@').expect("No @").0;

    map.insert(at_sym, '#');
    map.insert(at_sym.step(Dir::U), '#');
    map.insert(at_sym.step(Dir::D), '#');
    map.insert(at_sym.step(Dir::L), '#');
    map.insert(at_sym.step(Dir::R), '#');
    let points = [
        at_sym.up().left(),
        at_sym.up().right(),
        at_sym.down().left(),
        at_sym.down().right()
    ];
    solve(&map,&points)
}
pub fn solve(map: &HashMap<Point,char>, starts: &[Point]) -> usize {
    let key_count = map.iter().filter(|(_, c)| c.is_lowercase()).count();
    let mut known_bests: HashMap<(Vec<Point>, BTreeSet<char>), usize> = HashMap::new();
    known_bests.insert((starts.to_vec(), BTreeSet::new()), 0);
    for _ in 0..key_count {
        let mut new_known_bests : HashMap<(Vec<Point>, BTreeSet<char>), usize> = HashMap::new();
        for ((poss, keys), v) in known_bests.iter() {
            for (ix,bot) in poss.iter().enumerate(){
                let available_keys = bfs(&map, *bot, &keys);
                for (new_k, (d, p)) in available_keys.iter() {
                    let mut new_keys = keys.clone();
                    new_keys.insert(*new_k);
                    let mut new_ps = poss.clone();
                    new_ps[ix] = *p;
                    new_known_bests
                        .entry((new_ps, new_keys))
                        .and_modify(|e| *e = min(*e, d + v))
                        .or_insert(d + v);
                }
            }
        }
        known_bests = new_known_bests;
    }
    return *known_bests.values().min().expect("No answers?");
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
