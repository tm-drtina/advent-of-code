use std::str::FromStr;

use pathfinding::directed::astar::astar;

struct Map {
    raw_map: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_map: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let height = raw_map.len();
        let width = raw_map[0].len();

        Ok(Map {
            raw_map,
            height,
            width,
        })
    }
}

impl Map {
    fn goal(&self) -> (usize, usize) {
        (self.width - 1, self.height - 1)
    }

    fn cost(&self, point: (usize, usize)) -> usize {
        self.raw_map[point.1][point.0]
    }

    fn neighbors(&self, point: &(usize, usize)) -> Vec<((usize, usize), usize)> {
        let mut neighbors = Vec::new();

        if point.0 > 0 {
            let coord = (point.0 - 1, point.1);
            neighbors.push((coord, self.cost(coord)));
        }
        if point.0 < self.width - 1 {
            let coord = (point.0 + 1, point.1);
            neighbors.push((coord, self.cost(coord)));
        }
        if point.1 > 0 {
            let coord = (point.0, point.1 - 1);
            neighbors.push((coord, self.cost(coord)));
        }
        if point.1 < self.height - 1 {
            let coord = (point.0, point.1 + 1);
            neighbors.push((coord, self.cost(coord)));
        }

        neighbors
    }

    fn heurictics(&self, point: &(usize, usize)) -> usize {
        self.width - point.0 - 1 + self.height - point.1 - 1
    }
}

pub fn run(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    let res = astar(
        &(0, 0),
        |point| map.neighbors(point),
        |point| map.heurictics(point),
        |point| *point == map.goal(),
    )
    .expect("No path found");

    res.1
}
