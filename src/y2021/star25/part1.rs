#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    South,
    East,
}

struct Map(Vec<Vec<Tile>>);

impl std::str::FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '>' => Tile::East,
                        'v' => Tile::South,
                        '.' => Tile::Empty,
                        _ => unreachable!("invalid map char"),
                    })
                    .collect()
            })
            .collect();
        Ok(Self(data))
    }
}

impl Map {
    fn step_east(&mut self) -> bool {
        let mut res = false;
        for row in self.0.iter_mut() {
            let width = row.len();
            let indices: Vec<_> = (0..row.len())
                .filter(|i| row[*i] == Tile::East && row[(*i + 1) % width] == Tile::Empty)
                .collect();
            for i in indices {
                res = true;
                row[i] = Tile::Empty;
                row[(i + 1) % width] = Tile::East;
            }
        }
        res
    }

    fn step_south(&mut self) -> bool {
        let mut res = false;
        let height = self.0.len();
        for col in 0..self.0[0].len() {
            let indices: Vec<_> = (0..height)
                .filter(|i| {
                    self.0[*i][col] == Tile::South && self.0[(*i + 1) % height][col] == Tile::Empty
                })
                .collect();
            for i in indices {
                res = true;
                self.0[i][col] = Tile::Empty;
                self.0[(i + 1) % height][col] = Tile::South;
            }
        }
        res
    }

    fn step(&mut self) -> bool {
        let res1 = self.step_east();
        let res2 = self.step_south();
        res1 || res2
    }
}

pub fn run(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();
    for i in 1.. {
        if !map.step() {
            return i;
        }
    }
    0
}
