struct Map {
    data: Vec<Vec<bool>>,
}

struct Point {
    x: usize,
    y: usize,
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

    fn width(&self) -> usize {
        self.data.first().unwrap().len()
    }
    fn height(&self) -> usize {
        self.data.len()
    }

    fn count_trees(&self, step_down: usize, step_right: usize) -> usize {
        (0..)
            .map(|i| Point {
                x: i * step_right % self.width(),
                y: i * step_down,
            })
            .take_while(|p| p.y < self.height())
            .map(|p| self.data[p.y][p.x])
            .filter(|x| *x)
            .count()
    }
}

pub fn run(input: &str) -> usize {
    Map::new(input).count_trees(1, 3)
}
