use crate::utils::gcd;

struct Map {
    data: Vec<Vec<bool>>,
}

impl Map {
    fn new(input: &str) -> Self {
        Self {
            data: input
                .lines()
                .map(|line| line.chars().map(|ch| ch == '#').collect())
                .collect(),
        }
    }

    fn height(&self) -> usize {
        self.data.len()
    }
    fn width(&self) -> usize {
        self.data.first().unwrap().len()
    }

    fn is_visible(&self, ax: usize, ay: usize, bx: usize, by: usize) -> bool {
        let dx = bx as i32 - ax as i32;
        let dy = by as i32 - ay as i32;
        let k = gcd(dx.abs(), dy.abs());
        for i in 1..k {
            if self.data[(ay as i32 + dy / k * i) as usize][(ax as i32 + dx / k * i) as usize] {
                return false;
            }
        }
        true
    }

    fn detect(&self, px: usize, py: usize) -> usize {
        (0_usize..self.height())
            .flat_map(|y| (0_usize..self.width()).map(move |x| (x, y)))
            .filter(|(x, y)| px != *x || py != *y)
            .filter(|(x, y)| self.data[*y][*x])
            .filter(|(x, y)| self.is_visible(px, py, *x, *y))
            .count()
    }
}

pub fn run(input: &str) -> usize {
    let map = Map::new(input);
    (0..map.height())
        .flat_map(|y| (0_usize..map.width()).map(move |x| (x, y)))
        .filter(|(x, y)| map.data[*y][*x])
        .map(|(x, y)| map.detect(x, y))
        .max()
        .unwrap()
}
