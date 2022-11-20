#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

impl Point2D {
    pub fn top_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    pub fn top(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn top_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn bottom_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    pub fn bottom(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn bottom_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

pub struct Map<T> {
    map: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Map<T> {
    pub fn from_str<M>(input: &str, mut mapper: M) -> Self
    where
        M: FnMut(char, usize, usize) -> T,
    {
        let mut width = 0;
        let mut height = 0;

        let map = input
            .lines()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                width = line.len();
                height = y + 1;

                line.chars()
                    .enumerate()
                    .map(|(x, ch)| mapper(ch, x, y))
                    .collect::<Vec<T>>()
            })
            .collect();

        Self { map, width, height }
    }

    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.height
    }
    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set(&mut self, point: Point2D, value: T) {
        self.map[point.y][point.x] = value;
    }

    pub fn at(&self, point: Point2D) -> &T {
        &self.map[point.y][point.x]
    }

    pub fn four_neighborhood(&self, point: Point2D) -> Vec<Point2D> {
        let mut points = Vec::with_capacity(4);
        if point.x > 0 {
            points.push(Point2D { x: point.x - 1, y: point.y });
        }

        if point.x < self.width - 1 {
            points.push(Point2D { x: point.x + 1, y: point.y });
        }

        if point.y > 0 {
            points.push(Point2D { x: point.x, y: point.y - 1 });
        }

        if point.y < self.height {
            points.push(Point2D { x: point.x, y: point.y + 1 });
        }

        points
    }
}
