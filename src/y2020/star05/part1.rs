struct Seat {
    row: i32,
    col: i32,
}

impl Seat {
    fn id(&self) -> i32 {
        self.row * 8 + self.col
    }
}

fn parse_row(s: &str) -> i32 {
    (0..7)
        .filter(|i| s.chars().nth(*i).unwrap() == 'B')
        .map(|i| 2_i32.pow((6 - i) as u32))
        .sum()
}
fn parse_col(s: &str) -> i32 {
    (0..3)
        .filter(|i| s.chars().nth(*i + 7).unwrap() == 'R')
        .map(|i| 2_i32.pow((2 - i) as u32))
        .sum()
}

pub fn run(input: &str) -> i32 {
    input
        .lines()
        .map(|line| Seat {
            row: parse_row(line),
            col: parse_col(line),
        })
        .map(|seat| seat.id())
        .max()
        .unwrap()
}
