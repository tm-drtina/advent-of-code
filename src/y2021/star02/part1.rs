pub fn run(input: &str) -> usize {
    let (dist, depth) = input.lines().fold((0, 0), |(dist, depth), line| {
        if let Some(num) = line.strip_prefix("forward ") {
            (dist + num.parse::<usize>().unwrap(), depth)
        } else if let Some(num) = line.strip_prefix("down ") {
            (dist, depth + num.parse::<usize>().unwrap())
        } else if let Some(num) = line.strip_prefix("up ") {
            (dist, depth - num.parse::<usize>().unwrap())
        } else {
            panic!("Unknown input: '{}'", line)
        }
    });

    dist * depth
}
