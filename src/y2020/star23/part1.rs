use itertools::Itertools;

pub fn run(input: &str) -> String {
    let mut nums: Vec<i32> = input.chars().map(|ch| ch as i32 - '0' as i32).collect();

    for _ in 0..100 {
        let cur = nums[0];
        let mut dest = cur - 1;
        if dest == 0 {
            dest = 9
        }
        while dest == nums[1] || dest == nums[2] || dest == nums[3] {
            dest -= 1;
            if dest == 0 {
                dest = 9
            }
        }
        let mut next_nums: Vec<i32> = Vec::with_capacity(9);
        for x in &nums[4..] {
            next_nums.push(*x);
            if *x == dest {
                next_nums.push(nums[1]);
                next_nums.push(nums[2]);
                next_nums.push(nums[3]);
            }
        }
        next_nums.push(nums[0]);
        nums = next_nums;
    }

    nums.iter()
        .cycle()
        .skip_while(|x| **x != 1)
        .skip(1)
        .take(8)
        .join("")
}
