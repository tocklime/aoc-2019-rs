use num::{One, Zero};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub fn bfs_dist_all<N, C, FN, IN>(start: &N, mut successors: FN) -> HashMap<N, C>
where
    N: Eq + Hash + Copy,
    C: Zero + Ord + Copy + One,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let mut points: VecDeque<(N, C)> = VecDeque::new();
    points.push_back((start.clone(), C::zero()));
    let mut min_dist_map: HashMap<N, C> = HashMap::new();
    min_dist_map.insert(start.clone(), C::zero());
    while !points.is_empty() {
        let (pos, count) = points.pop_front().unwrap();
        for (p2, c) in successors(&pos) {
            if !min_dist_map.contains_key(&p2) {
                min_dist_map.insert(p2, count + c);
                let t = (p2.clone(), count + c);
                points.push_back(t);
            }
        }
    }
    min_dist_map
}

pub fn to_lookup<I, K, V>(tuples: I) -> HashMap<K, Vec<V>>
where
    I: IntoIterator<Item = (K, V)>,
    K: Eq + Hash,
{
    let mut m = HashMap::new();
    for (k, v) in tuples {
        m.entry(k).or_insert_with(Vec::new).push(v)
    }
    m
}
