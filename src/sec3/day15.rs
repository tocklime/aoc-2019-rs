use std::collections::HashMap;
use std::convert::TryInto;

use crate::comp::Computer;
use crate::utils::points::{Dir, Point};

const WALL: char = '█';
const SPACE: char = '.';
const EXPLORED_SPACE: char = ' ';
const UNKNOWN: char = '░';
const OXYGEN: char = 'O';
const DEAD_END: char = 'D';
const BRANCH: char = '╳';
const START: char = 'S';

const RESPS: [char; 3] = [WALL, SPACE, OXYGEN];
pub fn try_move(c: &mut Computer<i32>, d: Dir) -> (char, bool) {
    let i = match d {
        Dir::U => 1,
        Dir::D => 2,
        Dir::L => 3,
        Dir::R => 4,
    };
    c.with_input(i).run_to_input();
    let o = c.take_output();
    assert_eq!(o.len(), 1);
    let o_u: usize = o[0].try_into().unwrap();
    (RESPS[o_u], o_u > 0)
}

/// Performs a breadth first search of the map, and returns a map of point to distance from the start.
pub fn bfs_depth(map: &HashMap<Point, char>, start: Point) -> HashMap<Point, u32> {
    let mut points = std::collections::VecDeque::new();
    points.push_back((start, 0));
    let mut min_dist_map = HashMap::new();
    min_dist_map.insert(start, 0);
    while !points.is_empty() {
        let (pos, count) = points.pop_front().unwrap();
        Dir::all().iter().for_each(|d| {
            let p2 = pos + d.as_point_delta();
            if map.get(&p2) != Some(&WALL) {
                if !min_dist_map.contains_key(&p2) {
                    min_dist_map.insert(p2, count + 1);
                    let t = (p2, count + 1);
                    points.push_back(t);
                }
            }
        });
    }
    min_dist_map
}

/// Returns a map of the explored area. chars used:
///  * '.': space, but not explored yet. Should not occur in output.
///  * ' ': space, after exploration.
///  * 'X': space, was a choice point.
///  * 'D': space, was a dead end.
///  * 'O': space, containing oxygen generator
///  * 'S': space, starting position
///  * '#': wall
#[aoc_generator(day15)]
pub fn explore(input: &str) -> HashMap<Point, char> {
    let mut c = input.parse::<Computer>().unwrap();
    let mut known_map = HashMap::new();
    let mut position = Point(0, 0);
    known_map.insert(position, START);
    let mut save_points: Vec<(Point, Dir, Computer<i32>)> = Vec::new();
    loop {
        known_map.entry(position).and_modify(|x| {
            if *x == SPACE {
                *x = EXPLORED_SPACE;
            }
        });
        //scan around in directions we don't know.
        for &d in &Dir::all() {
            let new_pos = position.step(d);
            if !known_map.contains_key(&new_pos) {
                let (ch, moved) = try_move(&mut c, d);
                if moved {
                    try_move(&mut c, d.about_turn());
                }
                known_map.insert(new_pos, ch);
            }
        }
        //pick new dir
        let dirs: Vec<_> = Dir::all()
            .iter()
            .cloned()
            .filter(|&d| known_map[&position.step(d)] == SPACE)
            .collect();
        let heading = if dirs.is_empty() {
            known_map.insert(position, DEAD_END);
            match save_points.pop() {
                Some(s) => {
                    c = s.2;
                    position = s.0;
                    s.1
                }
                None => break known_map,
            }
        } else {
            for &other in dirs.iter().skip(1) {
                known_map.insert(position, BRANCH);
                save_points.push((position, other, c.clone()));
            }
            dirs[0]
        };
        assert_ne!(try_move(&mut c, heading), (WALL, false)); //already explored, so know this is safe.
        position = position.step(heading);
    }
}

#[aoc(day15, part1)]
pub fn p1(input: &HashMap<Point, char>) -> u32 {
    let (o_pos, _) = input
        .iter()
        .find(|(_, &v)| v == OXYGEN)
        .expect("No oxygen!");
    bfs_depth(input, Point(0, 0))[o_pos]
}
#[aoc(day15, part2)]
pub fn p2(input: &HashMap<Point, char>) -> u32 {
    let (&o_pos, _) = input
        .iter()
        .find(|(_, &v)| v == OXYGEN)
        .expect("No oxygen!");
    println!(
        "{}",
        crate::utils::points::render_char_map_w(input, 2, UNKNOWN)
    );
    *bfs_depth(input, o_pos).values().max().unwrap()
}
