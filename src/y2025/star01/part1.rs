use anyhow::Result;

fn to_rotation(line: &str) -> u8 {
    let (dir, amount) = line.split_at(1);
    let amount = amount.parse::<u32>().unwrap();
    match dir {
        "L" => 100 - ((amount % 100) as u8),
        "R" => (amount % 100) as u8,
        _ => panic!("Invalid format"),
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(to_rotation)
        .fold((50, 0), |(pos, count), rot| {
            let pos = (pos + rot) % 100;
            let count = count + if pos == 0 { 1 } else { 0 };
            (pos, count)
        })
        .1)
}
