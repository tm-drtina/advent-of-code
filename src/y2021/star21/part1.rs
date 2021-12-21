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

struct Game {
    dice: Dice,
    pos1: usize,
    score1: usize,
    pos2: usize,
    score2: usize,
}

impl Game {
    fn new(dice: Dice, pos1: usize, pos2: usize) -> Self {
        Self {
            dice,
            pos1: pos1 - 1,
            score1: 0,
            pos2: pos2 - 1,
            score2: 0,
        }
    }

    fn step(&mut self) {
        let throw = self.dice.next().unwrap();
        self.pos1 = (self.pos1 + throw) % 10;
        self.score1 += self.pos1 + 1;
        std::mem::swap(&mut self.pos1, &mut self.pos2);
        std::mem::swap(&mut self.score1, &mut self.score2);
    }

    fn run(&mut self) -> usize {
        let mut turns = 1;
        loop {
            self.step();
            if self.score2 >= 1000 {
                break;
            }
            turns += 1;
        };
        turns * 3 * self.score1
    }
}

pub fn run(input: &str) -> usize {
    let mut input = input.lines();
    let mut game = Game::new(
        Dice::new(100),
        input
            .next()
            .unwrap()
            .strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
        input
            .next()
            .unwrap()
            .strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
    );

    game.run()
}
