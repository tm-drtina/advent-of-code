pub fn run(input: &str, preamble_size: usize) -> i64 {
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let target = (0..nums.len() - preamble_size - 1)
        .map(|i| (nums[i + preamble_size], &nums[i..i + preamble_size]))
        .find(|(target, preamble)| {
            !(0..preamble.len())
                .any(|i| (i + 1..preamble.len()).any(|j| preamble[i] + preamble[j] == *target))
        })
        .unwrap()
        .0;

    let mut start_index = 0;
    let mut end_index = 0;
    let mut sum = nums[0];
    loop {
        if sum == target && end_index - start_index > 0 {
            break;
        }
        if sum > target {
            sum -= nums[start_index];
            start_index += 1;
        } else {
            end_index += 1;
            sum += nums[end_index]
        }
    }

    nums[start_index..end_index + 1].iter().min().unwrap()
        + nums[start_index..end_index + 1].iter().max().unwrap()
}
