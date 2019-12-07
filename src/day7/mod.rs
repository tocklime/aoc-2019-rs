use super::comp::Computer;
use itertools::Itertools;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[aoc_generator(day7)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn p1(input: &[isize]) -> isize {
    (0..5)
        .permutations(5)
        .map(|x| run_comp_loop(input, &x))
        .max()
        .unwrap()
}

#[test]
pub fn p1_tests() {
    let e0 = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    assert_eq!(p1(&gen(e0)), 43210);
}

#[aoc(day7, part2)]
pub fn p2(input: &[isize]) -> isize {
    (5..10)
        .permutations(5)
        .map(|x| run_comp_loop(input, &x))
        .max()
        .unwrap()
}
fn run_comp_loop(input: &[isize], a: &Vec<isize>) -> isize {
    let c_count = a.len();
    let comps: Vec<_> = std::iter::repeat_with(|| Arc::new(Mutex::new(Computer::new(input))))
        .take(c_count)
        .collect();
    for (ix, v) in a.iter().enumerate() {
        let (tx, rx) = mpsc::channel::<isize>();
        tx.send(*v).expect("Failed to set init value");
        if ix == 0 {
            tx.send(0).expect("Failed to set input");
        }
        comps[ix]
            .lock()
            .unwrap()
            .with_chan_input(rx)
            .with_name(format!("C-{}-{}", ix, v));
        comps[(ix + c_count - 1) % c_count]
            .lock()
            .unwrap()
            .with_chan_output(tx);
    }
    let ts: Vec<_> = comps
        .iter()
        .map(|c| {
            let cc = c.clone();
            thread::spawn(move || {
                let mut m = cc.lock().unwrap();
                m.run();
            })
        })
        .collect();
    for t in ts {
        t.join().unwrap();
    }
    let last_comp = comps.last().unwrap().lock().unwrap();
    last_comp.get_output()
}

#[test]
pub fn p2_tests() {
    //super::utils::log::enable_logging();
    let e0 =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    assert_eq!(run_comp_loop(&gen(e0), &vec![9, 8, 7, 6, 5]), 139629729);
    assert_eq!(p2(&gen(e0)), 139629729);
    let e1= "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    assert_eq!(run_comp_loop(&gen(e1), &vec![9, 7, 8, 5, 6]), 18216);
    assert_eq!(p2(&gen(e1)), 18216);
}
