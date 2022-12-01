pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let mut best = 0;
    let mut current = 0;
    while let Some(s) = lines.next() {
        if s.is_empty() {
            if current > best {
                best = current;
            }
            current = 0;
        } else {
            current += s.parse::<usize>().unwrap();
        }
    }
    std::cmp::max(best, current)
}
