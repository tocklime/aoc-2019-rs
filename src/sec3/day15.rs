use crate::utils::points::Dir;
use crate::utils::prelude::*;

pub fn try_move(c: &mut Computer<i32>, d: Dir) -> i32 {
    let i = match d {
        Dir::U => 1,
        Dir::D => 2,
        Dir::L => 3,
        Dir::R => 4,
    };
    c.with_input(i).run_to_input();
    let o = c.take_output();
    assert_eq!(o.len(), 1);
    o[0]
}

/// Performs a breadth first search of the map, and returns a map of point to distance from the start.
pub fn bfs_depth(map: &HashMap<Point, char>, start: Point) -> HashMap<Point, u32> {
    let mut points = std::collections::VecDeque::new();
    points.push_back((start, 0));
    let mut min_dist_map = HashMap::new();
    min_dist_map.insert(start, 0);
    while points.len() > 0 {
        let (pos, count) = points.pop_front().unwrap();
        Dir::all().iter().for_each(|d| {
            let p2 = pos + d.as_point_delta();
            if !(map.get(&p2) == Some(&'#')) {
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
    known_map.insert(position, 'S');
    let mut save_points: Vec<(Point, Dir, Computer<i32>)> = Vec::new();
    loop {
        if known_map.get(&position) == Some(&'.') {
            known_map.insert(position, ' ');
        }
        //scan around in directions we don't know.
        for &d in &Dir::all() {
            let new_pos = position + d.as_point_delta();
            if !known_map.contains_key(&new_pos) {
                match try_move(&mut c, d) {
                    0 => {
                        known_map.insert(new_pos, '#');
                    }
                    1 => {
                        known_map.insert(new_pos, '.');
                        try_move(&mut c, d.rotate_right().rotate_right()); //back where we started.
                    }
                    2 => {
                        known_map.insert(new_pos, 'O');
                        try_move(&mut c, d.rotate_right().rotate_right()); //back where we started.
                    }
                    _ => unreachable!(),
                }
            }
        }
        //pick new dir
        let dirs: Vec<_> = Dir::all()
            .iter()
            .cloned()
            .filter(|d| known_map[&(position + d.as_point_delta())] == '.')
            .collect();
        let heading = match dirs.len() {
            0 => match save_points.pop() {
                Some(s) => {
                    known_map.insert(position, 'D');
                    c = s.2;
                    position = s.0;
                    s.1
                }
                None => break known_map,
            },
            1 => dirs[0],
            _ => {
                for &other in dirs.iter().skip(1) {
                    save_points.push((position, other, c.clone()));
                }
                known_map.insert(position, 'X');
                dirs[0]
            }
        };
        try_move(&mut c, heading); //already explored, so know this is safe.
        position += heading.as_point_delta();
    }
}

#[aoc(day15, part1)]
pub fn p1(input: &HashMap<Point, char>) -> u32 {
    let (o_pos, _) = input.iter().find(|(_, &v)| v == 'O').expect("No oxygen!");
    bfs_depth(input, Point(0, 0))[o_pos]
}
#[aoc(day15, part2)]
pub fn p2(input: &HashMap<Point, char>) -> u32 {
    let (&o_pos, _) = input.iter().find(|(_, &v)| v == 'O').expect("No oxygen!");
    *bfs_depth(input, o_pos).values().max().unwrap()
}
