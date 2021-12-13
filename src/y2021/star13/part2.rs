use itertools::Itertools;

use super::part1::{parse, Point};

pub fn run(input: &str) -> String {
    let (mut points, folds) = parse(input);
    for fold in folds {
        points = points.into_iter().map(|pt| pt.fold(fold)).collect();
    }
    let res = (0..6)
        .map(|y| {
            (0..39)
                .map(|x| {
                    if points.contains(&Point(x, y)) {
                        "#"
                    } else {
                        " "
                    }
                })
                .join("")
        })
        .join("\n");
    eprintln!("{}", res);
    res
}
