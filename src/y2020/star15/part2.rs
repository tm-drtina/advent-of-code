pub fn run(input: &str) -> usize {
    let init: Vec<usize> = input.split(',').map(|line| line.parse().unwrap()).collect();

    let mut hist: Vec<Option<usize>> = vec![None; 30_000_000];

    for index in 0..(init.len() - 1) {
        hist[init[index]] = Some(index);
    }
    let mut last_num = init[init.len() - 1];
    for index in (init.len() - 1)..29_999_999 {
        let new_number = hist[last_num].map_or(0, |last| index - last);
        hist[last_num] = Some(index);
        last_num = new_number;
    }

    last_num
}
