use crate::utils::algorithms::{bfs_dist_all, to_lookup};
use crate::utils::points::{as_point_map, Point};
use crate::utils::prelude::HashMap;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Telepad {
    pos: Point,
    depth_change: isize,
}

#[aoc(day20, part1)]
pub fn p1(input: &str) -> u32 {
    solve(input, 0)
}
#[aoc(day20, part2)]
pub fn p2(input: &str) -> u32 {
    solve(input, 1)
}
pub fn solve(input: &str, depth_step: isize) -> u32 {
    let maz = as_point_map(input);
    let width = input.lines().nth(0).unwrap().len() as isize;
    let height = input.lines().count() as isize;
    let telepads = to_lookup::<_, String, Telepad>(maz.iter().filter_map(|(p, c)| {
        if c.is_ascii_alphabetic() {
            let n: Option<Vec<_>> = p.neighbours().iter().map(|x| maz.get(x)).collect();
            if let Some(n) = n {
                let a = if n[0].is_ascii_alphabetic() && n[2] == &'.' {
                    Some((format!("{}{}", c, n[0]), p.down()))
                } else if n[2].is_ascii_alphabetic() && n[0] == &'.' {
                    Some((format!("{}{}", n[2], c), p.up()))
                } else if n[1].is_ascii_alphabetic() && n[3] == &'.' {
                    Some((format!("{}{}", n[1], c), p.right()))
                } else if n[3].is_ascii_alphabetic() && n[1] == &'.' {
                    Some((format!("{}{}", c, n[3]), p.left()))
                } else {
                    None
                };
                let is_outer = p.0 < 3 || ((width - p.0) < 3) || p.1 < 3 || ((height - p.1) < 3);
                a.map(|(name, pos)| {
                    (
                        name.to_owned(),
                        Telepad {
                            pos,
                            depth_change: if is_outer { -1 } else { 1 },
                        },
                    )
                })
            } else {
                None
            }
        } else {
            None
        }
    }));
    let teleports: HashMap<Point, (Point, isize)> = telepads
        .values()
        .filter(|vs| vs.len() == 2)
        .flat_map(|vs| {
            vec![
                (vs[0].pos, (vs[1].pos, vs[0].depth_change)),
                (vs[1].pos, (vs[0].pos, vs[1].depth_change)),
            ]
        })
        .collect();

    let start = telepads["AA"][0].pos;
    let end = telepads["ZZ"][0].pos;
    let walking: HashMap<Point, HashMap<Point, (u32, isize)>> = telepads
        .values()
        .flatten()
        .map(|tp| {
            let s: HashMap<Point, u32> = bfs_dist_all(&tp.pos, |p| {
                p.neighbours()
                    .iter()
                    .filter(|n| maz.get(n) == Some(&'.'))
                    .map(|&n| (n, 1))
                    .collect_vec()
            });
            let filtered = s
                .iter()
                .filter_map(|(p, &dist)| {
                    if *p == end {
                        Some((*p, (dist, 0)))
                    } else {
                        teleports
                            .get(p)
                            .map(|(p, dc)| (*p, (dist + 1, *dc * depth_step)))
                    }
                })
                .collect::<HashMap<Point, (u32, isize)>>();
            (tp.pos, filtered)
        })
        .collect();
    dijkstra(
        &(start, 0),
        |(pos, depth)| {
            walking[pos]
                .iter()
                .filter_map(|(p, (dist, dc))| {
                    let new_depth = depth + dc;
                    if new_depth >= 0 {
                        Some(((*p, new_depth), *dist))
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
        |(p, d)| *p == end && *d == 0,
    )
    .expect("No solution")
    .1
}

#[cfg(test)]
mod test {
    #[test]
    pub fn d20p1tests() {
        let a = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
";
        assert_eq!(super::p1(a), 58);
    }

    #[test]
    pub fn d20p2tests() {
        let triv = "       A       
       A       
  #####.#####  
  #####.#####  
AB..###.#####  
  #.## A ####  
BC..## BC....ZZ
  ###########  
  ###########  
               
               
";
        let a = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
";
        assert_eq!(super::p2(triv), 11);
        assert_eq!(super::p2(a), 396);
    }
}
