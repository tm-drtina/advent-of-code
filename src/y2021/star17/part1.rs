pub(super) fn parse(input: &str) -> ((i32, i32), (i32, i32)) {
    let input = input.strip_prefix("target area: ").unwrap();
    let (x, y) = input.split_once(", ").unwrap();
    let x = x.strip_prefix("x=").unwrap();
    let (x_from, x_to) = x.split_once("..").unwrap();
    let y = y.strip_prefix("y=").unwrap();
    let (y_from, y_to) = y.split_once("..").unwrap();

    (
        (x_from.parse().unwrap(), x_to.parse().unwrap()),
        (y_from.parse().unwrap(), y_to.parse().unwrap()),
    )
}

pub(super) fn check(target: &((i32, i32), (i32, i32)), x_speed: i32, y_speed: i32) -> bool {
    let mut n = if y_speed >= 0 { 2 * y_speed + 2 } else { 1 };
    loop {
        let capped_n = n.min(x_speed);
        let x = capped_n * (2 * x_speed - capped_n + 1) / 2;
        let y = n * y_speed - n * (n - 1) / 2;

        if x > target.0.1 || y < target.1.0 {
            return false;
        }
        if x >= target.0.0 && y <= target.1.1 {
            return true;
        }
        n += 1;
    }
}

pub fn run(input: &str) -> i32 {
    let target = parse(input);

    let min_x = 1;
    let max_x = target.0.1 + 1;
    let max_y = -target.1.0;
    let mut best_y_speed = 0;

    for x_speed in min_x..max_x {
        if x_speed * (x_speed + 1) / 2 < target.0.0 {
            continue;
        }
        for y_speed in ((best_y_speed + 1)..max_y).rev() {
            if y_speed <= best_y_speed {
                break;
            }
            if check(&target, x_speed, y_speed) {
                best_y_speed = y_speed;
            }
        }
    }

    i32::midpoint(best_y_speed * best_y_speed, best_y_speed)
}
