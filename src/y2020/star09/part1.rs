pub fn run(input: &str) -> i64 {
    let mut lines = input.lines();
    let preamble_size = lines.next().unwrap().parse::<usize>().unwrap();
    let nums: Vec<i64> = lines.map(|line| line.parse().unwrap()).collect();

    (0..nums.len() - preamble_size - 1)
        .map(|i| (nums[i + preamble_size], &nums[i..i + preamble_size]))
        .find(|(target, preamble)| {
            !(0..preamble.len())
                .any(|i| (i + 1..preamble.len()).any(|j| preamble[i] + preamble[j] == *target))
        })
        .unwrap()
        .0
}
