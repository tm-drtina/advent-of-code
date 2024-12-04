use crate::utils::point::Point2D;

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

    pub fn set(&mut self, point: Point2D<usize>, value: T) {
        self.map[point.y][point.x] = value;
    }

    pub fn at(&self, point: Point2D<usize>) -> &T {
        &self.map[point.y][point.x]
    }

    pub fn four_neighborhood(&self, point: Point2D<usize>) -> Vec<Point2D<usize>> {
        let mut points = Vec::with_capacity(4);
        if point.x > 0 {
            points.push(Point2D {
                x: point.x - 1,
                y: point.y,
            });
        }

        if point.x < self.width - 1 {
            points.push(Point2D {
                x: point.x + 1,
                y: point.y,
            });
        }

        if point.y > 0 {
            points.push(Point2D {
                x: point.x,
                y: point.y - 1,
            });
        }

        if point.y < self.height - 1 {
            points.push(Point2D {
                x: point.x,
                y: point.y + 1,
            });
        }

        points
    }

    pub fn iter_values(&self) -> impl Iterator<Item = (Point2D<usize>, &T)> {
        self.map.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, val)| (Point2D { x, y }, val))
        })
    }

    pub fn is_valid_point(&self, pt: &Point2D<usize>) -> bool {
        pt.x < self.width && pt.y < self.height
    }
}
