pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let mut values = Vec::new();
    let mut current = 0;
    while let Some(s) = lines.next() {
        if s.is_empty() {
            values.push(current);
            current = 0;
        } else {
            current += s.parse::<usize>().unwrap();
        }
    }
    values.push(current);
    values.sort_unstable();
    values.iter().rev().take(3).sum()
}
