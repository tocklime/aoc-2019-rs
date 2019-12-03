use super::day3::*;
use super::utils::points::*;
#[test]
pub fn wire_intersections() {
    let w1 = WireLine {
        dir: Dir::U,
        start: Point(0, 0),
        len: 10,
        signal_delay: 0,
    };
    let w2 = WireLine {
        dir: Dir::R,
        start: Point(-3, 4),
        len: 10,
        signal_delay: 0,
    };
    let w3 = WireLine {
        dir: Dir::U,
        start: Point(1, 0),
        len: 10,
        signal_delay: 0,
    };
    assert_eq!(Some(Point(0, 4)), w1.intersects(&w2));
    assert_eq!(None, w1.intersects(&w3));
    let a1 = WireLine {
        dir: Dir::R,
        len: 75,
        start: Point(0, 0),
        signal_delay: 0,
    };
    let a2 = WireLine {
        dir: Dir::R,
        len: 66,
        start: Point(0, 62),
        signal_delay: 62,
    };
    assert_eq!(None, a1.intersects(&a2)); //(Point(66, 62)) at Point(0, 62)
}

#[test]
pub fn tests() {
    let h0 = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let h1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let h2 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    //p1
    assert_eq!(p1(&gen(h0)), 6);
    assert_eq!(p1(&gen(h1)), 159);
    assert_eq!(p1(&gen(h2)), 135);
    //p2
    assert_eq!(p2(&gen(h0)), 30);
    assert_eq!(p2(&gen(h1)), 610);
    assert_eq!(p2(&gen(h2)), 410);
}
