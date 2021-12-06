use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(usize, usize);
impl Point {
    fn from_raw(raw: &str) -> Self {
        let (x, y) = raw.split_once(',').unwrap();
        Self(x.parse().unwrap(), y.parse().unwrap())
    }
}

pub fn run(input: &str) -> usize {
    let mut map = HashMap::<Point, usize>::new();
    for line in input.lines() {
        let (raw_pt1, raw_pt2) = line.split_once(" -> ").unwrap();
        let pt1 = Point::from_raw(raw_pt1);
        let pt2 = Point::from_raw(raw_pt2);
        if pt1.0 != pt2.0 && pt1.1 != pt2.1 {
            continue;
        }
        for x in if pt1.0 <= pt2.0 {
            pt1.0..=pt2.0
        } else {
            pt2.0..=pt1.0
        } {
            for y in if pt1.1 <= pt2.1 {
                pt1.1..=pt2.1
            } else {
                pt2.1..=pt1.1
            } {
                *map.entry(Point(x, y)).or_default() += 1;
            }
        }
    }

    map.values().filter(|v| **v > 1).count()
}
