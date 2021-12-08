pub fn run(input: &str) -> i32 {
    let mut lines = input.lines();
    let min_time = lines.next().unwrap().parse::<i32>().unwrap();
    lines
        .next()
        .unwrap()
        .split(',')
        .filter(|ch| *ch != "x")
        .map(|line| line.parse::<i32>().unwrap())
        .map(|line| (line, (0..).find(|x| x * line >= min_time).unwrap() * line))
        .min_by_key(|(_line, time)| *time)
        .map(|(line, time)| (time - min_time) * line)
        .unwrap()
}
