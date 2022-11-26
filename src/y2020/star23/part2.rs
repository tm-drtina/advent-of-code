use std::collections::VecDeque;

type Num = u32;

fn pop_front(nums: &mut VecDeque<Num>, to_insert: &mut [Vec<Num>]) -> Num {
    let res = nums.pop_front().unwrap();
    let values = &mut to_insert[res as usize];
    for x in values.iter() {
        nums.push_front(*x);
    }
    values.clear();
    res
}

pub fn run(input: &str) -> usize {
    let mut nums = input
        .chars()
        .map(|ch| ch as Num - '0' as Num)
        .chain(10..=1_000_000)
        .collect();

    let mut to_insert: Vec<Vec<Num>> = (0..=1_000_000).map(|_| Vec::new()).collect();

    for _ in 0..10_000_000 {
        let n0 = pop_front(&mut nums, &mut to_insert);
        let n1 = pop_front(&mut nums, &mut to_insert);
        let n2 = pop_front(&mut nums, &mut to_insert);
        let n3 = pop_front(&mut nums, &mut to_insert);

        let mut dest = n0 - 1;
        if dest == 0 {
            dest = 1_000_000;
        }
        while dest == n1 || dest == n2 || dest == n3 {
            dest -= 1;
            if dest == 0 {
                dest = 1_000_000;
            }
        }

        to_insert[dest as usize].push(n1);
        to_insert[n1 as usize].push(n2);
        to_insert[n2 as usize].push(n3);

        nums.push_back(n0);
    }

    while pop_front(&mut nums, &mut to_insert) != 1 {
        // run
    }

    pop_front(&mut nums, &mut to_insert) as usize * pop_front(&mut nums, &mut to_insert) as usize
}
