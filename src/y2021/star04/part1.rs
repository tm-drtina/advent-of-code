use std::collections::{HashMap, HashSet};

const BOARD_SIZE: usize = 5;

struct Card {
    marked_rows: [usize; BOARD_SIZE],
    marked_cols: [usize; BOARD_SIZE],
    unmarked: HashSet<usize>,
    nums: HashMap<usize, (usize, usize)>,
}

impl Card {
    fn from_raw(rows: [&str; BOARD_SIZE]) -> Self {
        let mut unmarked = HashSet::new();
        let mut nums = HashMap::new();

        for (row_index, raw_row) in rows.into_iter().enumerate() {
            for (col_index, s) in raw_row.split(' ').filter(|s| !s.is_empty()).enumerate() {
                let num: usize = s.parse().unwrap();
                unmarked.insert(num);
                nums.insert(num, (row_index, col_index));
            }
        }
        debug_assert_eq!(BOARD_SIZE*BOARD_SIZE, nums.len());
        debug_assert_eq!(BOARD_SIZE*BOARD_SIZE, unmarked.len());

        Self {
            marked_rows: [0; BOARD_SIZE],
            marked_cols: [0; BOARD_SIZE],
            unmarked,
            nums,
        }
    }

    fn draw(&mut self, number: usize) -> Option<usize> {
        if let Some((row, col)) = self.nums.get(&number) {
            self.marked_rows[*row] += 1;
            self.marked_cols[*col] += 1;
            self.unmarked.remove(&number);

            if self.marked_rows[*row] == BOARD_SIZE || self.marked_cols[*col] == BOARD_SIZE {
                return Some(self.unmarked.iter().copied().sum::<usize>() * number);
            }
        }
        None
    }
}

pub fn run(input: &str) -> usize {
    let mut input = input.lines();
    let numbers: Vec<usize> = input
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cards = Vec::<Card>::new();

    let mut line = input.next();
    while line.is_some() {
        if line.unwrap().is_empty() {
            line = input.next();
            continue;
        }
        let raw_cards = [
            line.unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        ];
        cards.push(Card::from_raw(raw_cards));
        line = input.next();
    }

    for number in numbers {
        for card in cards.iter_mut() {
            if let Some(score) = card.draw(number) {
                return score;
            }
        }
    }

    panic!("No solution found!");
}
