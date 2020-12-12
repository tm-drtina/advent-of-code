#[derive(Copy, Clone, Debug)]
enum Seat {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

struct Map {
    positions: Vec<Vec<Seat>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        Self {
            positions: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| if ch == 'L' { Seat::EMPTY } else { Seat::FLOOR })
                        .collect()
                })
                .collect(),
        }
    }
}

impl Map {
    fn height(&self) -> usize {
        self.positions.len()
    }
    fn width(&self) -> usize {
        self.positions[0].len()
    }
    fn collect_neighbors(&self, x: usize, y: usize) -> Vec<Seat> {
        let mut res: Vec<Seat> = Vec::new();
        if y > 0 {
            res.push(self.positions[y - 1][x])
        }
        if y < self.height() - 1 {
            res.push(self.positions[y + 1][x])
        }
        if x > 0 {
            res.push(self.positions[y][x - 1])
        }
        if x < self.width() - 1 {
            res.push(self.positions[y][x + 1])
        }
        if y > 0 && x > 0 {
            res.push(self.positions[y - 1][x - 1])
        }
        if y < self.height() - 1 && x > 0 {
            res.push(self.positions[y + 1][x - 1])
        }
        if y > 0 && x < self.width() - 1 {
            res.push(self.positions[y - 1][x + 1])
        }
        if y < self.height() - 1 && x < self.width() - 1 {
            res.push(self.positions[y + 1][x + 1])
        }
        res
    }

    fn next_evolution(&self) -> (bool, Self) {
        let mut did_change = false;
        let new = Self {
            positions: (0..self.height())
                .map(|y| {
                    (0..self.width())
                        .map(|x| match self.positions[y][x] {
                            Seat::FLOOR => Seat::FLOOR,
                            Seat::EMPTY => {
                                if self
                                    .collect_neighbors(x, y)
                                    .iter()
                                    .all(|seat| !matches!(seat, Seat::OCCUPIED))
                                {
                                    did_change = true;
                                    Seat::OCCUPIED
                                } else {
                                    Seat::EMPTY
                                }
                            }
                            Seat::OCCUPIED => {
                                if self
                                    .collect_neighbors(x, y)
                                    .iter()
                                    .filter(|seat| matches!(seat, Seat::OCCUPIED))
                                    .count()
                                    >= 4
                                {
                                    did_change = true;
                                    Seat::EMPTY
                                } else {
                                    Seat::OCCUPIED
                                }
                            }
                        })
                        .collect()
                })
                .collect(),
        };
        (did_change, new)
    }

    fn run_evolutions(self) -> Self {
        let mut current = self;
        loop {
            let (did_change, next) = current.next_evolution();
            current = next;
            if !did_change {
                return current;
            }
        }
    }
}

pub fn run(input: &str) -> usize {
    Map::from(input)
        .run_evolutions()
        .positions
        .iter()
        .flat_map(|line| line)
        .filter(|seat| matches!(seat, Seat::OCCUPIED))
        .count()
}
