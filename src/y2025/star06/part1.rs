pub fn run(input: &str) -> u64 {
    let mut lines = input.lines().collect::<Vec<_>>();
    let ops: Vec<_> = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|ch| match ch {
            "*" => <u64 as std::ops::MulAssign>::mul_assign,
            "+" => <u64 as std::ops::AddAssign>::add_assign,
            _ => panic!(),
        })
        .collect();

    lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .reduce(|mut a, b| {
            a.iter_mut()
                .zip(b)
                .zip(ops.iter())
                .for_each(|((x, y), op)| op(x, y));
            a
        })
        .unwrap()
        .into_iter()
        .sum()
}
