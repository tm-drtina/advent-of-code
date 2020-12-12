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

        let mut neigh_y = y;
        while neigh_y > 0 && matches!(self.positions[neigh_y - 1][x], Seat::FLOOR) {
            neigh_y -= 1
        }
        if neigh_y > 0 {
            res.push(self.positions[neigh_y - 1][x])
        }

        let mut neigh_y = y;
        while neigh_y < self.height() - 1 && matches!(self.positions[neigh_y + 1][x], Seat::FLOOR) {
            neigh_y += 1
        }
        if neigh_y < self.height() - 1 {
            res.push(self.positions[neigh_y + 1][x])
        }

        let mut neigh_x = x;
        while neigh_x > 0 && matches!(self.positions[y][neigh_x - 1], Seat::FLOOR) {
            neigh_x -= 1
        }
        if neigh_x > 0 {
            res.push(self.positions[y][neigh_x - 1])
        }

        let mut neigh_x = x;
        while neigh_x < self.width() - 1 && matches!(self.positions[y][neigh_x + 1], Seat::FLOOR) {
            neigh_x += 1
        }
        if neigh_x < self.width() - 1 {
            res.push(self.positions[y][neigh_x + 1])
        }

        let mut neigh_y = y;
        let mut neigh_x = x;
        while neigh_y > 0
            && neigh_x > 0
            && matches!(self.positions[neigh_y - 1][neigh_x - 1], Seat::FLOOR)
        {
            neigh_y -= 1;
            neigh_x -= 1;
        }
        if neigh_y > 0 && neigh_x > 0 {
            res.push(self.positions[neigh_y - 1][neigh_x - 1])
        }

        let mut neigh_y = y;
        let mut neigh_x = x;
        while neigh_y < self.height() - 1
            && neigh_x > 0
            && matches!(self.positions[neigh_y + 1][neigh_x - 1], Seat::FLOOR)
        {
            neigh_y += 1;
            neigh_x -= 1;
        }
        if neigh_y < self.height() - 1 && neigh_x > 0 {
            res.push(self.positions[neigh_y + 1][neigh_x - 1])
        }

        let mut neigh_y = y;
        let mut neigh_x = x;
        while neigh_y > 0
            && neigh_x < self.width() - 1
            && matches!(self.positions[neigh_y - 1][neigh_x + 1], Seat::FLOOR)
        {
            neigh_y -= 1;
            neigh_x += 1;
        }
        if neigh_y > 0 && neigh_x < self.width() - 1 {
            res.push(self.positions[neigh_y - 1][neigh_x + 1])
        }

        let mut neigh_y = y;
        let mut neigh_x = x;
        while neigh_y < self.height() - 1
            && neigh_x < self.width() - 1
            && matches!(self.positions[neigh_y + 1][neigh_x + 1], Seat::FLOOR)
        {
            neigh_y += 1;
            neigh_x += 1;
        }
        if neigh_y < self.height() - 1 && neigh_x < self.width() - 1 {
            res.push(self.positions[neigh_y + 1][neigh_x + 1])
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
                                    >= 5
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
