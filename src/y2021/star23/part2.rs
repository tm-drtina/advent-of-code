use once_cell::sync::Lazy;
use pathfinding::prelude::dijkstra;

/*
01 2 3 4 56
  7 8 9 0
  1 2 3 4
  5 6 7 8
  9 0 1 2
*/
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Puzzle {
    data: [u8; 23],
}

struct Path {
    steps: usize,
    avoid: Vec<usize>,
}
const RESULT: [u8; 23] = [
    0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4,
];
const COSTS: [usize; 4] = [1, 10, 100, 1000];
static PATHS: Lazy<[Vec<Path>; 4]> = Lazy::new(|| {
    [
        vec![
            Path {
                steps: 3,
                avoid: vec![1],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 4,
                avoid: vec![2],
            },
            Path {
                steps: 6,
                avoid: vec![2, 3],
            },
            Path {
                steps: 8,
                avoid: vec![2, 3, 4],
            },
            Path {
                steps: 9,
                avoid: vec![2, 3, 4, 5],
            },
        ],
        vec![
            Path {
                steps: 5,
                avoid: vec![1, 2],
            },
            Path {
                steps: 4,
                avoid: vec![2],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 4,
                avoid: vec![3],
            },
            Path {
                steps: 6,
                avoid: vec![3, 4],
            },
            Path {
                steps: 7,
                avoid: vec![3, 4, 5],
            },
        ],
        vec![
            Path {
                steps: 7,
                avoid: vec![1, 2, 3],
            },
            Path {
                steps: 6,
                avoid: vec![2, 3],
            },
            Path {
                steps: 4,
                avoid: vec![3],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 4,
                avoid: vec![4],
            },
            Path {
                steps: 5,
                avoid: vec![4, 5],
            },
        ],
        vec![
            Path {
                steps: 9,
                avoid: vec![1, 2, 3, 4],
            },
            Path {
                steps: 8,
                avoid: vec![2, 3, 4],
            },
            Path {
                steps: 6,
                avoid: vec![3, 4],
            },
            Path {
                steps: 4,
                avoid: vec![4],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 2,
                avoid: vec![],
            },
            Path {
                steps: 3,
                avoid: vec![5],
            },
        ],
    ]
});

impl std::str::FromStr for Puzzle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut data = [0; 23];

        lines.next().unwrap();
        lines.next().unwrap();
        let mut line = lines.next().unwrap().chars();
        data[7] = line.nth(3).unwrap() as u8 - b'A' + 1;
        data[8] = line.nth(1).unwrap() as u8 - b'A' + 1;
        data[9] = line.nth(1).unwrap() as u8 - b'A' + 1;
        data[10] = line.nth(1).unwrap() as u8 - b'A' + 1;

        data[11] = 4;
        data[12] = 3;
        data[13] = 2;
        data[14] = 1;

        data[15] = 4;
        data[16] = 2;
        data[17] = 1;
        data[18] = 3;

        let mut line = lines.next().unwrap().chars();
        data[19] = line.nth(3).unwrap() as u8 - b'A' + 1;
        data[20] = line.nth(1).unwrap() as u8 - b'A' + 1;
        data[21] = line.nth(1).unwrap() as u8 - b'A' + 1;
        data[22] = line.nth(1).unwrap() as u8 - b'A' + 1;

        Ok(Self { data })
    }
}

impl Puzzle {
    fn successors(&self) -> Vec<(Puzzle, usize)> {
        let mut res = Vec::new();
        for i in 0..7 {
            if self.data[i] == 0 {
                continue;
            }
            let path = &PATHS[(self.data[i] - 1) as usize][i];
            let cost = COSTS[(self.data[i] - 1) as usize];
            let target = self.data[i] as usize + 6;
            if path.avoid.iter().any(|index| self.data[*index] != 0) {
                continue;
            }
            if self.data[target] == 0
                && self.data[target + 4] == 0
                && self.data[target + 8] == 0
                && self.data[target + 12] == 0
            {
                let mut new_state = self.clone();
                new_state.data[target + 12] = self.data[i];
                new_state.data[i] = 0;
                res.push((new_state, (path.steps + 3) * cost));
            } else if self.data[target] == 0
                && self.data[target + 4] == 0
                && self.data[target + 8] == 0
                && self.data[target + 12] == self.data[i]
            {
                let mut new_state = self.clone();
                new_state.data[target + 8] = self.data[i];
                new_state.data[i] = 0;
                res.push((new_state, (path.steps + 2) * cost));
            } else if self.data[target] == 0
                && self.data[target + 4] == 0
                && self.data[target + 8] == self.data[i]
                && self.data[target + 12] == self.data[i]
            {
                let mut new_state = self.clone();
                new_state.data[target + 4] = self.data[i];
                new_state.data[i] = 0;
                res.push((new_state, (path.steps + 1) * cost));
            } else if self.data[target] == 0
                && self.data[target + 4] == self.data[i]
                && self.data[target + 8] == self.data[i]
                && self.data[target + 12] == self.data[i]
            {
                let mut new_state = self.clone();
                new_state.data[target] = self.data[i];
                new_state.data[i] = 0;
                res.push((new_state, path.steps * cost));
            }
        }
        for i in 0..4 {
            if self.data[i + 7] == 0
                || (self.data[i + 7] - 1 == i as u8
                    && self.data[i + 11] - 1 == i as u8
                    && self.data[i + 15] - 1 == i as u8
                    && self.data[i + 19] - 1 == i as u8)
            {
                continue;
            }
            for target in 0..7 {
                if self.data[target] != 0 {
                    continue;
                }
                let path = &PATHS[i][target];
                let cost = COSTS[(self.data[i + 7] - 1) as usize];
                if path.avoid.iter().any(|index| self.data[*index] != 0) {
                    continue;
                }
                let mut new_state = self.clone();
                new_state.data[target] = self.data[i + 7];
                new_state.data[i + 7] = 0;
                res.push((new_state, path.steps * cost));
            }
        }
        for i in 0..4 {
            if self.data[i + 7] != 0
                || self.data[i + 11] == 0
                || (self.data[i + 11] - 1 == i as u8
                    && self.data[i + 15] - 1 == i as u8
                    && self.data[i + 19] - 1 == i as u8)
            {
                continue;
            }
            for target in 0..7 {
                if self.data[target] != 0 {
                    continue;
                }
                let path = &PATHS[i][target];
                let cost = COSTS[(self.data[i + 11] - 1) as usize];
                if path.avoid.iter().any(|index| self.data[*index] != 0) {
                    continue;
                }
                let mut new_state = self.clone();
                new_state.data[target] = self.data[i + 11];
                new_state.data[i + 11] = 0;
                res.push((new_state, (path.steps + 1) * cost));
            }
        }
        for i in 0..4 {
            if self.data[i + 7] != 0
                || self.data[i + 11] != 0
                || self.data[i + 15] == 0
                || (self.data[i + 15] - 1 == i as u8 && self.data[i + 19] - 1 == i as u8)
            {
                continue;
            }
            for target in 0..7 {
                if self.data[target] != 0 {
                    continue;
                }
                let path = &PATHS[i][target];
                let cost = COSTS[(self.data[i + 15] - 1) as usize];
                if path.avoid.iter().any(|index| self.data[*index] != 0) {
                    continue;
                }
                let mut new_state = self.clone();
                new_state.data[target] = self.data[i + 15];
                new_state.data[i + 15] = 0;
                res.push((new_state, (path.steps + 2) * cost));
            }
        }
        for i in 0..4 {
            if self.data[i + 7] != 0
                || self.data[i + 11] != 0
                || self.data[i + 15] != 0
                || self.data[i + 19] == 0
                || self.data[i + 19] - 1 == i as u8
            {
                continue;
            }
            for target in 0..7 {
                if self.data[target] != 0 {
                    continue;
                }
                let path = &PATHS[i][target];
                let cost = COSTS[(self.data[i + 19] - 1) as usize];
                if path.avoid.iter().any(|index| self.data[*index] != 0) {
                    continue;
                }
                let mut new_state = self.clone();
                new_state.data[target] = self.data[i + 19];
                new_state.data[i + 19] = 0;
                res.push((new_state, (path.steps + 3) * cost));
            }
        }
        res
    }
    fn success(&self) -> bool {
        self.data == RESULT
    }
}

pub fn run(input: &str) -> usize {
    let start: Puzzle = input.parse().unwrap();
    dijkstra(&start, Puzzle::successors, Puzzle::success)
        .unwrap()
        .1
}
