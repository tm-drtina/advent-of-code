pub fn run(input: &str) -> usize {
    let (dist, depth, _) = input
        .lines()
        .fold((0, 0, 0), |(dist, depth, aim), line| {
            if let Some(num) = line.strip_prefix("forward ") {
                let new_dist: usize = num.parse().unwrap();
                (dist + new_dist, depth + aim * new_dist, aim)
            } else if let Some(num) = line.strip_prefix("down ") {
                (dist, depth, aim + num.parse::<usize>().unwrap())
            } else if let Some(num) = line.strip_prefix("up ") {
                (dist, depth, aim - num.parse::<usize>().unwrap())
            } else {
                panic!("Unknown input: '{}'", line)
            }
        });

    dist * depth
}
