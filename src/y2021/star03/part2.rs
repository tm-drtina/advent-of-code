fn compute_bit_hist(input: &[&str], index: usize) -> i32 {
    input.iter().fold(0, |acc, s| {
        if s.chars().nth(index).unwrap() == '0' {
            acc - 1
        } else if s.chars().nth(index).unwrap() == '1' {
            acc + 1
        } else {
            panic!()
        }
    })
}

pub fn run(input: &str) -> usize {
    let input: Vec<&str> = input.lines().collect();
    let bit_len = input[0].len();
    let ox_bin = {
        let mut input = input.clone();
        for i in 0..bit_len {
            let ch = if compute_bit_hist(&input, i) < 0 {
                '0'
            } else {
                '1'
            };
            input = input
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == ch)
                .collect();
            if input.len() == 1 {
                break;
            }
        }
        input[0]
    };
    let co_bin = {
        let mut input = input.clone();
        for i in 0..bit_len {
            let ch = if compute_bit_hist(&input, i) < 0 {
                '1'
            } else {
                '0'
            };
            input = input
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == ch)
                .collect();
            if input.len() == 1 {
                break;
            }
        }
        input[0]
    };

    let mut ox = 0;
    let mut co = 0;

    for i in 0..bit_len {
        ox <<= 1;
        co <<= 1;
        if ox_bin.chars().nth(i).unwrap() == '1' {
            ox += 1;
        }
        if co_bin.chars().nth(i).unwrap() == '1' {
            co += 1;
        }
    }

    ox * co
}
