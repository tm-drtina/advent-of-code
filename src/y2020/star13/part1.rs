use std::str::FromStr;

pub fn run(input: &str) -> i32 {
    let mut lines = input.lines();
    let min_time = i32::from_str(lines.next().unwrap()).unwrap();
    lines
        .next()
        .unwrap()
        .split(',')
        .filter(|ch| *ch != "x")
        .map(|line| i32::from_str(line).unwrap())
        .map(|line| {
            (
                line,
                (0..).find(|x| x * line >= min_time).unwrap() * line,
            )
        })
        .min_by_key(|(_line, time)| *time)
        .map(|(line, time)| (time - min_time) * line)
        .unwrap()
}
