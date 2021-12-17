use super::part1::{check, parse};

pub fn run(input: &str) -> usize {
    let target = parse(input);

    let min_x = 1;
    let max_x = target.0 .1 + 1;
    let min_y = target.1 .0;
    let max_y = -target.1 .0;
    let mut count = 0;

    for x_speed in min_x..max_x {
        if x_speed * (x_speed + 1) / 2 < target.0 .0 {
            continue;
        }
        for y_speed in min_y..max_y {
            if check(&target, x_speed, y_speed) {
                count += 1;
            }
        }
    }

    count
}
