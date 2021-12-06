use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(i32, i32);
impl Point {
    fn from_raw(raw: &str) -> Self {
        let (x, y) = raw.split_once(',').unwrap();
        Self(x.parse().unwrap(), y.parse().unwrap())
    }
}

pub fn run(input: &str) -> usize {
    let mut map = HashMap::<Point, i32>::new();
    for line in input.lines() {
        let (raw_pt1, raw_pt2) = line.split_once(" -> ").unwrap();
        let pt1 = Point::from_raw(raw_pt1);
        let pt2 = Point::from_raw(raw_pt2);
        let x_inc = match pt1.0.cmp(&pt2.0) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        let y_inc = match pt1.1.cmp(&pt2.1) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        let mut pt = pt1;
        loop {
            *map.entry(pt).or_default() += 1;
            if pt == pt2 {
                break;
            }
            pt = Point(pt.0 + x_inc, pt.1 + y_inc);
        }
    }

    map.values().filter(|v| **v > 1).count()
}
