use std::slice::Iter;

use num::{Integer, Signed, Unsigned};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dir {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Dir {
    pub fn clockwise_90(self) -> Self {
        match self {
            Self::TopLeft => Self::TopRight,
            Self::Top => Self::Right,
            Self::TopRight => Self::BottomRight,
            Self::Right => Self::Bottom,
            Self::BottomRight => Self::BottomLeft,
            Self::Bottom => Self::Left,
            Self::BottomLeft => Self::TopLeft,
            Self::Left => Self::Top,
        }
    }

    pub fn counterclockwise_90(self) -> Self {
        match self {
            Self::TopLeft => Self::BottomLeft,
            Self::Top => Self::Left,
            Self::TopRight => Self::TopLeft,
            Self::Right => Self::Top,
            Self::BottomRight => Self::TopRight,
            Self::Bottom => Self::Right,
            Self::BottomLeft => Self::BottomRight,
            Self::Left => Self::Bottom,
        }
    }

    pub fn iter() -> Iter<'static, Self> {
        static DIRECTIONS: [Dir; 8] = [
            Dir::TopLeft,
            Dir::Top,
            Dir::TopRight,
            Dir::Right,
            Dir::BottomRight,
            Dir::Bottom,
            Dir::BottomLeft,
            Dir::Left,
        ];
        DIRECTIONS.iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2D<Coord: Integer> {
    pub x: Coord,
    pub y: Coord,
}

impl<Coord: Integer + Copy> Point2D<Coord> {
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
}

impl<Coord: Integer + Copy + Signed> Point2D<Coord> {
    #[allow(dead_code)]
    pub fn step_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::TopLeft => self.top_left(),
            Dir::Top => self.top(),
            Dir::TopRight => self.top_right(),
            Dir::Right => self.right(),
            Dir::BottomRight => self.bottom_right(),
            Dir::Bottom => self.bottom(),
            Dir::BottomLeft => self.bottom_left(),
            Dir::Left => self.left(),
        }
    }

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

    #[allow(dead_code)]
    pub fn four_neighborhood(&self) -> [Self; 4] {
        [self.top(), self.right(), self.bottom(), self.left()]
    }

    pub fn eight_neighborhood(&self) -> [Self; 8] {
        [
            self.top_left(),
            self.top(),
            self.top_right(),
            self.right(),
            self.bottom_right(),
            self.bottom(),
            self.bottom_left(),
            self.left(),
        ]
    }
}

impl<Coord: Integer + Copy + Unsigned> Point2D<Coord> {
    #[allow(dead_code)]
    pub fn try_step_dir(&self, dir: Dir) -> Option<Self> {
        match dir {
            Dir::TopLeft => self.try_top_left(),
            Dir::Top => self.try_top(),
            Dir::TopRight => self.try_top_right(),
            Dir::Right => Some(self.right()),
            Dir::BottomRight => Some(self.bottom_right()),
            Dir::Bottom => Some(self.bottom()),
            Dir::BottomLeft => self.try_bottom_left(),
            Dir::Left => self.try_left(),
        }
    }

    pub fn try_top_left(&self) -> Option<Self> {
        if self.x.is_zero() || self.y.is_zero() {
            return None;
        }
        Some(Self {
            x: self.x - Coord::one(),
            y: self.y - Coord::one(),
        })
    }

    pub fn try_top(&self) -> Option<Self> {
        if self.y.is_zero() {
            return None;
        }
        Some(Self {
            x: self.x,
            y: self.y - Coord::one(),
        })
    }

    pub fn try_top_right(&self) -> Option<Self> {
        if self.y.is_zero() {
            return None;
        }
        Some(Self {
            x: self.x + Coord::one(),
            y: self.y - Coord::one(),
        })
    }

    pub fn try_bottom_left(&self) -> Option<Self> {
        if self.x.is_zero() {
            return None;
        }
        Some(Self {
            x: self.x - Coord::one(),
            y: self.y + Coord::one(),
        })
    }

    pub fn try_left(&self) -> Option<Self> {
        if self.x.is_zero() {
            return None;
        }
        Some(Self {
            x: self.x - Coord::one(),
            y: self.y,
        })
    }

    #[allow(dead_code)]
    pub fn try_four_neighborhood(&self) -> Vec<Self> {
        vec![
            self.try_top(),
            Some(self.right()),
            Some(self.bottom()),
            self.try_left(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[allow(dead_code)]
    pub fn try_eight_neighborhood(&self) -> Vec<Self> {
        vec![
            self.try_top_left(),
            self.try_top(),
            self.try_top_right(),
            Some(self.right()),
            Some(self.bottom_right()),
            Some(self.bottom()),
            self.try_bottom_left(),
            self.try_left(),
        ]
        .into_iter()
        .flatten()
        .collect()
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
