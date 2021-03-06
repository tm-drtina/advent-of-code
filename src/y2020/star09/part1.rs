use std::str::FromStr;

pub fn run(input: &str, preamble_size: usize) -> i64 {
    let nums: Vec<i64> = input
        .lines()
        .map(|line| i64::from_str(line).unwrap())
        .collect();

    (0..nums.len() - preamble_size - 1)
        .map(|i| (nums[i + preamble_size], &nums[i..i + preamble_size]))
        .filter(|(target, preamble)| {
            !(0..preamble.len())
                .any(|i| (i + 1..preamble.len()).any(|j| preamble[i] + preamble[j] == *target))
        })
        .next()
        .unwrap()
        .0
}
