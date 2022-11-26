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
