#[derive(Copy, Clone, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied,
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
                        .map(|ch| if ch == 'L' { Seat::Empty } else { Seat::Floor })
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
                            Seat::Floor => Seat::Floor,
                            Seat::Empty => {
                                if self
                                    .collect_neighbors(x, y)
                                    .iter()
                                    .all(|seat| !matches!(seat, Seat::Occupied))
                                {
                                    did_change = true;
                                    Seat::Occupied
                                } else {
                                    Seat::Empty
                                }
                            }
                            Seat::Occupied => {
                                if self
                                    .collect_neighbors(x, y)
                                    .iter()
                                    .filter(|seat| matches!(seat, Seat::Occupied))
                                    .count()
                                    >= 4
                                {
                                    did_change = true;
                                    Seat::Empty
                                } else {
                                    Seat::Occupied
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
        .flatten()
        .filter(|seat| matches!(seat, Seat::Occupied))
        .count()
}
