use crate::utils::points::{as_point_map, Point};
use crate::utils::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Teleport<'a> {
    Unconnected(&'a Telepad),
    Connects(&'a Telepad, &'a Telepad),
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Telepad {
    input: Point,
    output: Point,
    name: String,
    depth_change: isize,
}

impl<'a> Teleport<'a> {
    pub fn connect(self, o: &'a Telepad) -> Self {
        match self {
            Teleport::Unconnected(p) => Teleport::Connects(p, o),
            Teleport::Connects(a, b) => panic!(
                "Teleport already connected from {:?} to {:?} so can't connect {:?}",
                a, b, o
            ),
        }
    }
    pub fn single(self) -> Point {
        match self {
            Teleport::Unconnected(p) => p.output,
            _ => panic!("Cannot get single of connected point"),
        }
    }
    pub fn teleport(self, p: Point) -> (Point, isize) {
        if let Teleport::Connects(a, b) = self {
            if p == a.input {
                (b.output, a.depth_change)
            } else if p == b.input {
                (a.output, b.depth_change)
            } else {
                panic!("Not this teleport")
            }
        } else {
            panic!("Teleport not connected")
        }
    }
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
            if map.get(&p2) == Some(&'.') && !min_dist_map.contains_key(&p2) {
                min_dist_map.insert(p2, count + 1);
                let t = (p2, count + 1);
                points.push_back(t);
            }
        });
    }
    min_dist_map
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
    let teleport_points: Vec<Telepad> = maz
        .iter()
        .filter_map(|(p, c)| {
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
                    let is_outer =
                        p.0 < 3 || ((width - p.0) < 3) || p.1 < 3 || ((height - p.1) < 3);
                    a.map(|(name, output)| Telepad {
                        input: *p,
                        output: output,
                        name: name,
                        depth_change: if is_outer {
                            -1 * depth_step
                        } else {
                            1 * depth_step
                        },
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let walking = teleport_points
        .iter()
        .map(|tp| {
            let s = bfs_depth(&maz, tp.output);
            let filtered = teleport_points
                .iter()
                .filter_map(|x| {
                    if x == tp {
                        return None;
                    }
                    if let Some(d) = s.get(&x.output) {
                        Some((x, *d))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>();
            (tp.output, filtered)
        })
        .collect::<HashMap<_, _>>();
    let mut teleport_names: HashMap<String, Teleport> = HashMap::new();
    for t in teleport_points.iter() {
        teleport_names
            .entry(t.name.to_owned())
            .and_modify(|x| *x = x.connect(t))
            .or_insert(Teleport::Unconnected(t));
    }
    let mut teleport_links: HashMap<Point, (Point, isize)> = HashMap::new();
    for t in teleport_names.values() {
        match t {
            Teleport::Unconnected(_) => (),
            Teleport::Connects(a, b) => {
                teleport_links.insert(a.input, (b.output, a.depth_change));
                teleport_links.insert(b.input, (a.output, b.depth_change));
            }
        }
    }
    let start = teleport_names["AA"].single();
    let end = teleport_names["ZZ"].single();

    let mut points = std::collections::VecDeque::new();
    points.push_back((start, 0, Vec::new()));
    //let mut min_dist_map:HashMap<(Point,isize), = HashMap::new();
    //min_dist_map.insert((start, 0), Vec::new());
    loop {
        let (pos, depth, path) = points.pop_front().unwrap();
        if pos == end && depth == 0 {
            break path.iter().map(|(_, _, dist)| dist).sum();
        }
        //opts from here:
        walking[&pos]
            .iter()
            .for_each(|(&tp, &dist)| match teleport_names.get(&tp.name) {
                Some(Teleport::Unconnected(_)) => {
                    if depth == 0 {
                        let mut path = path.clone();
                        path.push((tp.output, depth, dist));
                        points.push_back((tp.output, depth, path));
                    }
                }
                Some(t) => {
                    let (new_p, dc) = t.teleport(tp.input);
                    if depth + dc >= 0 {
                        let mut path = path.clone();
                        path.push((new_p, depth + dc, dist + 1));
                        points.push_back((new_p, depth + dc, path));
                    }
                }
                _ => {}
            })
    }
}
//6124 too low

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
    //   assert_eq!(p1(a), 58);
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
    assert_eq!(p2(triv), 11);
    assert_eq!(p2(a), 396);
}
