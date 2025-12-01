use anyhow::Result;

fn to_rotation(line: &str) -> i32 {
    let (dir, amount) = line.split_at(1);
    let amount = amount.parse::<i32>().unwrap();
    match dir {
        "L" => -amount,
        "R" => amount,
        _ => panic!("Invalid format"),
    }
}

pub fn run(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(to_rotation)
        .fold((50, 0), |(mut pos, mut count), rot| {
            let old_pos = pos;
            count += (rot / 100).abs();
            pos += rot % 100;
            if pos >= 100 {
                pos -= 100;
                count += 1;
            } else if pos < 0 {
                pos += 100;
                if old_pos > 0 {
                    count += 1;
                }
            } else if pos == 0 {
                count += 1;
            }
            (pos, count)
        })
        .1)
}
