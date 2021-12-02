pub fn run(input: &str) -> usize {
    let (dist, depth) = input
        .lines()
        .fold((0, 0), |(dist, depth), line| {
            if line.starts_with("forward ") {
                (dist + line[8..].parse::<usize>().unwrap(), depth)
            } else if line.starts_with("down ") {
                (dist, depth + line[5..].parse::<usize>().unwrap())
            } else if line.starts_with("up") {
                (dist, depth - line[3..].parse::<usize>().unwrap())
            } else {
                panic!("Unknown input: '{}'", line)
            }
        });

    dist * depth
}
