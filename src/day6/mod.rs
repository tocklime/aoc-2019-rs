use std::cell::RefCell;
use std::collections::HashMap;

struct OrbitalMap<'a> {
    map: HashMap<&'a str, &'a str>,
    depth_cache: RefCell<HashMap<&'a str, usize>>,
}

impl<'a> OrbitalMap<'a> {
    pub fn from_str(input: &'a str) -> OrbitalMap<'a> {
        OrbitalMap {
            map: input
                .lines()
                .map(|l| l.split(')').collect::<Vec<&str>>())
                .map(|a| (a[1], a[0]))
                .collect(),
            depth_cache: RefCell::new(HashMap::new()),
        }
    }
    fn get_chain_to_root(&self, obj: &'a str) -> Vec<&str> {
        let mut curr: Option<&&str> = Some(&obj);
        let mut vec: Vec<&str> = Vec::new();
        while let Some(x) = curr {
            vec.push(x);
            curr = self.map.get(x);
        }
        vec
    }
    pub fn get_depth(&'a self, obj: &'a str) -> usize {
        let mut dc = self.depth_cache.borrow_mut();
        dc.get(obj).cloned().unwrap_or_else(|| {
            let chain = self.get_chain_to_root(obj);
            for (ix, i) in chain.iter().enumerate() {
                if dc.contains_key(i) {
                    break;
                }
                dc.insert(i, chain.len() - ix);
            }
            chain.len()
        })
    }
}
#[aoc(day6, part1)]
pub fn p1(input: &str) -> usize {
    let ors = OrbitalMap::from_str(input);
    ors.map.values().map(|x| ors.get_depth(x)).sum()
}
#[aoc(day6, part2)]
pub fn p2(input: &str) -> usize {
    let ors = OrbitalMap::from_str(input);
    let my_chain = ors.get_chain_to_root("YOU");
    let san_chain = ors.get_chain_to_root("SAN");
    let common_prefix_len = my_chain
        .iter()
        .rev()
        .zip(san_chain.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    return my_chain.len() + san_chain.len() - 2 * (common_prefix_len + 1);
}

#[test]
pub fn p1_tests() {
    let h0 = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    assert_eq!(p1(h0), 42);
}

#[test]
pub fn p2_tests() {
    let h0 = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    assert_eq!(p2(h0), 4);
}
