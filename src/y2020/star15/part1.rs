use std::collections::HashMap;

pub fn run(input: &str) -> i32 {
    let init: Vec<i32> = input.split(',').map(|line| line.parse().unwrap()).collect();

    let mut last_num = init[0];
    let mut index = 0;
    let mut hist: HashMap<i32, i32> = HashMap::new();

    for x in &init[1..] {
        hist.insert(last_num, index);
        last_num = *x;
        index += 1;
    }
    while index < 2019 {
        let new_number = hist.get(&last_num).map_or(0, |last| index - last);
        hist.insert(last_num, index);
        last_num = new_number;
        index += 1;
    }

    last_num
}
