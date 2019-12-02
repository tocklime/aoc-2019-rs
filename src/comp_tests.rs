use super::comp::Computer;
use super::day2;
pub fn t(input: &str, out_ix: usize) -> usize {
    Computer::new(&day2::gen(input)).run().abs_load(out_ix)
}

#[test]
pub fn day2_tests() {
    assert_eq!(t("1,0,0,0,99", 0), 2);
    assert_eq!(t("2,3,0,3,99", 3), 6);
    assert_eq!(t("2,4,4,5,99,0", 5), 9801);
    assert_eq!(t("1,1,1,4,99,5,6,0,99", 0), 30);
}
