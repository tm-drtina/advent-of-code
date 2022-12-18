use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3D<Coord: Integer> {
    pub x: Coord,
    pub y: Coord,
    pub z: Coord,
}

impl<Coord: Integer + Copy> Point3D<Coord> {
    pub fn six_neighborhood(self) -> [Self; 6] {
        let Point3D { x, y, z } = self;
        [
            Point3D { x: x + Coord::one(), y, z },
            Point3D { x: x - Coord::one(), y, z },
            Point3D { x, y: y + Coord::one(), z },
            Point3D { x, y: y - Coord::one(), z },
            Point3D { x, y, z: z + Coord::one() },
            Point3D { x, y, z: z - Coord::one() },
        ]
    }
}

impl<Coord: Integer> Default for Point3D<Coord> {
    fn default() -> Self {
        Self {
            x: Coord::zero(),
            y: Coord::zero(),
            z: Coord::zero(),
        }
    }
}
impl<Coord: Integer> std::ops::Add for Point3D<Coord> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<Coord: Integer + Copy> std::ops::AddAssign for Point3D<Coord> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl Point3D<usize> {
    #[allow(dead_code)]
    pub fn manhattan_distance(&self) -> usize {
        self.x + self.y + self.z
    }
}
impl Point3D<i32> {
    #[allow(dead_code)]
    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
