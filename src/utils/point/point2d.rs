use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D<Coord: Integer> {
    pub x: Coord,
    pub y: Coord,
}

impl<Coord: Integer + Copy> Point2D<Coord> {
    pub fn top_left(&self) -> Self {
        Self {
            x: self.x - Coord::one(),
            y: self.y - Coord::one(),
        }
    }

    pub fn top(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - Coord::one(),
        }
    }

    pub fn top_right(&self) -> Self {
        Self {
            x: self.x + Coord::one(),
            y: self.y - Coord::one(),
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + Coord::one(),
            y: self.y,
        }
    }

    pub fn bottom_right(&self) -> Self {
        Self {
            x: self.x + Coord::one(),
            y: self.y + Coord::one(),
        }
    }

    pub fn bottom(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + Coord::one(),
        }
    }

    pub fn bottom_left(&self) -> Self {
        Self {
            x: self.x - Coord::one(),
            y: self.y + Coord::one(),
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - Coord::one(),
            y: self.y,
        }
    }
}

impl<Coord: Integer + Copy + PartialOrd> Point2D<Coord> {
    pub fn manhattan_distance(self, other: Self) -> Coord {
        let x_diff = if self.x < other.x {
            other.x - self.x
        } else {
            self.x - other.x
        };
        let y_diff = if self.y < other.y {
            other.y - self.y
        } else {
            self.y - other.y
        };
        x_diff + y_diff
    }
}

impl<Coord: Integer + Copy + Default> Default for Point2D<Coord> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
