use std::collections::{HashMap, LinkedList};

fn pop_front(nums: &mut LinkedList<i32>, to_insert: &mut HashMap<i32, Vec<i32>>) -> i32 {
    let res = nums.pop_front().unwrap();
    if to_insert.contains_key(&res) {
        for x in to_insert.get(&res).unwrap() {
            nums.push_front(*x);
        }
        to_insert.remove(&res);
    }
    res
}

pub fn run(input: &str) -> i64 {
    let input_nums: Vec<i32> = input.chars().map(|ch| ch as i32 - '0' as i32).collect();
    let mut nums: LinkedList<i32> = (10..1_000_001).collect();
    for i in 0..9 {
        nums.push_front(input_nums[8 - i]);
    }

    let mut to_insert: HashMap<i32, Vec<i32>> = HashMap::new();

    for _ in 0..10_000_000 {
        let n0 = pop_front(&mut nums, &mut to_insert);
        let n1 = pop_front(&mut nums, &mut to_insert);
        let n2 = pop_front(&mut nums, &mut to_insert);
        let n3 = pop_front(&mut nums, &mut to_insert);

        let mut dest = n0 - 1;
        if dest == 0 {
            dest = 1_000_000
        }
        while dest == n1 || dest == n2 || dest == n3 {
            dest -= 1;
            if dest == 0 {
                dest = 1_000_000
            }
        }

        to_insert.entry(dest).or_default().push(n1);
        to_insert.entry(n1).or_default().push(n2);
        to_insert.entry(n2).or_default().push(n3);

        nums.push_back(n0);
    }

    while pop_front(&mut nums, &mut to_insert) != 1 {
        // run
    }

    pop_front(&mut nums, &mut to_insert) as i64 * pop_front(&mut nums, &mut to_insert) as i64
}
