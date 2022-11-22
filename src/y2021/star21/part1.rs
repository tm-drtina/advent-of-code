struct Dice {
    size: usize,
    it: std::ops::RangeInclusive<usize>,
}

impl Dice {
    fn new(size: usize) -> Self {
        Self { size, it: 1..=size }
    }

    fn next_num(&mut self) -> usize {
        self.it.next().unwrap_or_else(|| {
            self.it = 1..=self.size;
            self.it.next().unwrap()
        })
    }
}

impl Iterator for Dice {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_num() + self.next_num() + self.next_num())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(super) struct Game {
    pub pos1: usize,
    pub score1: usize,
    pub pos2: usize,
    pub score2: usize,
    pub p1_turn: bool,
}

impl Game {
    fn new(pos1: usize, pos2: usize) -> Self {
        Self {
            pos1: pos1 - 1,
            score1: 0,
            pos2: pos2 - 1,
            score2: 0,
            p1_turn: true,
        }
    }

    pub(super) fn step(&mut self, throw: usize) {
        self.pos1 = (self.pos1 + throw) % 10;
        self.score1 += self.pos1 + 1;
        self.p1_turn = !self.p1_turn;
        std::mem::swap(&mut self.pos1, &mut self.pos2);
        std::mem::swap(&mut self.score1, &mut self.score2);
    }

    fn run(&mut self, mut dice: Dice) -> usize {
        let mut turns = 1;
        loop {
            let throw = dice.next().unwrap();
            self.step(throw);
            if self.score2 >= 1000 {
                break;
            }
            turns += 1;
        }
        turns * 3 * self.score1
    }
}

impl std::str::FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Self::new(
            lines
                .next()
                .unwrap()
                .strip_prefix("Player 1 starting position: ")
                .unwrap()
                .parse()
                .unwrap(),
            lines
                .next()
                .unwrap()
                .strip_prefix("Player 2 starting position: ")
                .unwrap()
                .parse()
                .unwrap(),
        ))
    }
}

pub fn run(input: &str) -> usize {
    let dice = Dice::new(100);
    let mut game: Game = input.parse().unwrap();

    game.run(dice)
}
