pub fn run(input: &str) -> usize {
    let mut values = Vec::new();
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            values.push(current);
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }
    values.push(current);
    values.sort_unstable();
    values.iter().rev().take(3).sum()
}
