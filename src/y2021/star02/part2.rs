pub fn run(input: &str) -> usize {
    let (dist, depth, _) = input
        .lines()
        .fold((0, 0, 0), |(dist, depth, aim), line| {
            if line.starts_with("forward ") {
                let new_dist: usize = line[8..].parse().unwrap();
                (dist + new_dist, depth + aim * new_dist, aim)
            } else if line.starts_with("down ") {
                (dist, depth, aim + line[5..].parse::<usize>().unwrap())
            } else if line.starts_with("up") {
                (dist, depth, aim - line[3..].parse::<usize>().unwrap())
            } else {
                panic!("Unknown input: '{}'", line)
            }
        });

    dist * depth
}
