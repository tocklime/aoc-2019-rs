use num::{One, Zero};
use std::collections::{HashMap, VecDeque, HashSet};
use std::hash::{Hash, BuildHasher};
use std::borrow::Borrow;

pub fn bfs_dist_all<N, C, FN, IN>(start: &N, mut successors: FN) -> HashMap<N, C>
    where
        N: Eq + Hash + Copy,
        C: Zero + Ord + Copy + One,
        FN: FnMut(&N) -> IN,
        IN: IntoIterator<Item=(N, C)>,
{
    let mut points: VecDeque<(N, C)> = VecDeque::new();
    points.push_back((*start, C::zero()));
    let mut min_dist_map: HashMap<N, C> = HashMap::new();
    min_dist_map.insert(start.clone(), C::zero());
    while !points.is_empty() {
        let (pos, count) = points.pop_front().unwrap();
        for (p2, c) in successors(&pos) {
            min_dist_map.entry(p2).or_insert_with(|| {
                points.push_back((p2, count + c));
                count + c
            });
        }
    }
    min_dist_map
}

pub fn to_lookup<I, K, V>(tuples: I) -> HashMap<K, Vec<V>>
    where
        I: IntoIterator<Item=(K, V)>,
        K: Eq + Hash,
{
    let mut m = HashMap::new();
    for (k, v) in tuples {
        m.entry(k).or_insert_with(Vec::new).push(v)
    }
    m
}

pub fn automata_step<T, FN, FC, Q, S>(g: &Q, neighbours: FN, check: FC) -> HashSet<T, S>
    where FN: Fn(T) -> Vec<T>,
          FC: Fn(bool, usize) -> bool,
          T: Ord + Copy + Hash,
          S: BuildHasher + Default,
          Q: Borrow<HashSet<T, S>>
{
    let mut counts: HashMap<T, usize> = HashMap::new();
    let g = g.borrow();
    for &p in g.iter() {
        for n in neighbours(p) {
            *counts.entry(n).or_default() += 1;
        }
    }
    counts.iter()
        .filter_map(|(p, &c)| {
            if check(g.contains(p), c) {
                Some(*p)
            } else { None }
        }).collect()
}
