use std::str::FromStr;

pub fn run(input: &str) -> i64 {
    let mut lines = input.lines();
    lines.next();
    let mut buses: Vec<(i64, i64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_index, ch)| *ch != "x")
        .map(|(index, bus)| (index as i64, i64::from_str(bus).unwrap()))
        .collect();

    buses.sort_by_key(|(_index, bus)| -bus);

    println!("{:?}", buses);

    let mut res = -buses[0].0;
    let mut n = buses[0].1;
    while res < 0 {
        res += n;
    }
    for (index, bus) in buses[1..].iter() {
        let mut tmp = -(*index);
        while tmp < 0 {
            tmp += bus
        }
        while res % bus != tmp {
            res += n;
        }
        n *= bus;
    }

    res
}
