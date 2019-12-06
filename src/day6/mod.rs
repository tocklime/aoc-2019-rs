use std::collections::HashMap;

struct OrbitalMap<'a> {
    map: HashMap<&'a str, &'a str>,
    depth_cache: HashMap<&'a str, usize>,
}

impl<'a> OrbitalMap<'a> {
    pub fn new() -> OrbitalMap<'a> {
        OrbitalMap {
            map: HashMap::new(),
            depth_cache: HashMap::new(),
        }
    }
    fn get_chain_to_root(&self, obj: &'a str) -> Vec<&str> {
        let mut curr: Option<&&str> = Some(&obj);
        let mut vec: Vec<&str> = Vec::new();
        loop {
            match curr {
                None => break,
                Some(x) => {
                    vec.push(x);
                    curr = self.map.get(x);
                }
            }
        }
        vec
    }
    pub fn get_depth(self: &'a Self, obj: &'a str) -> usize {
        self.get_chain_to_root(obj).len()
    }
    pub fn get_depth_cached(self: &'a mut Self, obj: &'a str) -> usize {
        match self.depth_cache.get(obj) {
            Some(&x) => x,
            None => {
                let chain = self.get_chain_to_root(obj);
                for (ix, i) in chain.iter().enumerate() {
                    //self.depth_cache.insert(i, chain.len() - ix);
                }
                chain.len()
            }
        }
    }
}
fn gen<'a>(input: &'a str) -> OrbitalMap<'a> {
    let mut ors = OrbitalMap::new();
    for l in input.lines() {
        let e: Vec<&str> = l.split(')').collect();
        ors.map.insert(e[1], e[0]);
    }
    ors
}
#[aoc(day6, part1)]
pub fn p1(input: &str) -> usize {
    let ors = gen(input);
    ors.map.values().map(|x| ors.get_depth(x)).sum()
}
#[aoc(day6, part2)]
pub fn p2(input: &str) -> usize {
    let mut ors = gen(input);
    let my_chain = ors.get_chain_to_root("YOU");
    let san_chain = ors.get_chain_to_root("SAN");
    let prefix = my_chain
        .iter()
        .rev()
        .zip(san_chain.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    return my_chain.len() + san_chain.len() - 2 * (prefix + 1);
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
