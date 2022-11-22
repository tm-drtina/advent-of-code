use crate::utils::SortedByAngle;

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

    fn detect(&self, px: usize, py: usize) -> Vec<(usize, usize)> {
        (0_usize..self.height())
            .flat_map(|y| (0_usize..self.width()).map(move |x| (x, y)))
            .filter(|(x, y)| px != *x || py != *y)
            .filter(|(x, y)| self.data[*y][*x])
            .filter(|(x, y)| self.is_visible(px, py, *x, *y))
            .collect()
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn run(input: &str) -> usize {
    let mut map = Map::new(input);

    let (center_x, center_y, _) = (0..map.height())
        .flat_map(|y| (0_usize..map.width()).map(move |x| (x, y)))
        .filter(|(x, y)| map.data[*y][*x])
        .map(|(x, y)| (x, y, map.detect(x, y).len()))
        .max_by_key(|x| x.2)
        .unwrap();

    let mut destroyed = 0;
    let mut detected = map.detect(center_x, center_y);

    while detected.len() + destroyed < 200 {
        destroyed += detected.len();
        for (x, y) in detected {
            map.data[y][x] = false;
        }
        detected = map.detect(center_x, center_y);
    }

    detected
        .into_iter()
        .sorted_by_angle(
            |x| (x.0 as i32 - center_x as i32, center_y as i32 - x.1 as i32), // FLIPPED Y AXIS!
            std::f64::consts::FRAC_PI_2,
        )
        .nth(199 - destroyed)
        .map(|p| p.1 + 100 * p.0)
        .unwrap()
}
