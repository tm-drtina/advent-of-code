pub fn run(input: &str) -> usize {
    let init: Vec<i32> = input.lines().next().unwrap().chars().map(|_| 0).collect();
    let hist = input
        .lines()
        .fold(init, |mut acc, line| {
            line.chars().enumerate().for_each(|(index, ch)| {
                if ch == '0' {
                    acc[index] -= 1;
                } else if ch == '1' {
                    acc[index] += 1;
                } else {
                    panic!("Invalid bit");
                }
            });
            acc
        });

    let mut gamma = 0;
    let mut epsilon = 0;

    for h in hist {
        match h.cmp(&0) {
            std::cmp::Ordering::Less => {
                gamma <<= 1;
                epsilon = (epsilon << 1) + 1;
            }
            std::cmp::Ordering::Greater => {
                gamma = (gamma << 1) + 1;
                epsilon <<= 1;
            }
            std::cmp::Ordering::Equal => {}
        };
    }

    gamma * epsilon
}
