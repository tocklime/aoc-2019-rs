use super::comp::{Computer, ComputerState};
use super::day2;
pub fn t(input: &str, out_ix: isize) -> isize {
    Computer::new(&day2::gen(input)).run().abs_load(out_ix)
}

#[test]
pub fn day2_tests() {
    assert_eq!(t("1,0,0,0,99", 0), 2);
    assert_eq!(t("2,3,0,3,99", 3), 6);
    assert_eq!(t("2,4,4,5,99,0", 5), 9801);
    assert_eq!(t("1,1,1,4,99,5,6,0,99", 0), 30);
}

pub fn t2(input: &str, i_val: isize) -> isize {
    let mem = &day2::gen(input);
    let mut c = Computer::new(mem);
    c.with_input(i_val);

    while c.state() == ComputerState::RUNNING {
        println!("{:?}", c);
        c.step();
    }
    c.get_output()
}
#[test]
pub fn io_tests() {
    assert_eq!(t2("3,9,8,9,10,9,4,9,99,-1,8", 7), 0);
    assert_eq!(t2("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);
    assert_eq!(t2("3,9,8,9,10,9,4,9,99,-1,8", 9), 0);
}
