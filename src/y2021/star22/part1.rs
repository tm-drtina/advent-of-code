#[derive(Debug, Clone, Copy)]
struct Range {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl std::str::FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let x_str = split
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .split_once("..")
            .unwrap();
        let x = (x_str.0.parse().unwrap(), x_str.1.parse().unwrap());
        let y_str = split
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .split_once("..")
            .unwrap();
        let y = (y_str.0.parse().unwrap(), y_str.1.parse().unwrap());
        let z_str = split
            .next()
            .unwrap()
            .strip_prefix("z=")
            .unwrap()
            .split_once("..")
            .unwrap();
        let z = (z_str.0.parse().unwrap(), z_str.1.parse().unwrap());
        Ok(Self { x, y, z })
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.x.0 <= other.x.0
            && self.x.1 >= other.x.1
            && self.y.0 <= other.y.0
            && self.y.1 >= other.y.1
            && self.z.0 <= other.z.0
            && self.z.1 >= other.z.1
    }
    fn intersects(&self, other: &Self) -> bool {
        self.x.1 >= other.x.0
            && self.x.0 <= other.x.1
            && self.y.1 >= other.y.0
            && self.y.0 <= other.y.1
            && self.z.1 >= other.z.0
            && self.z.0 <= other.z.1
    }

    fn size(&self) -> usize {
        ((self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)) as usize
    }

    fn split_x(self, coord: isize) -> (Self, Self) {
        (
            Self {
                x: (self.x.0, coord),
                y: self.y,
                z: self.z,
            },
            Self {
                x: (coord + 1, self.x.1),
                y: self.y,
                z: self.z,
            },
        )
    }
    fn split_y(self, coord: isize) -> (Self, Self) {
        (
            Self {
                x: self.x,
                y: (self.y.0, coord),
                z: self.z,
            },
            Self {
                x: self.x,
                y: (coord + 1, self.y.1),
                z: self.z,
            },
        )
    }
    fn split_z(self, coord: isize) -> (Self, Self) {
        (
            Self {
                x: self.x,
                y: self.y,
                z: (self.z.0, coord),
            },
            Self {
                x: self.x,
                y: self.y,
                z: (coord + 1, self.z.1),
            },
        )
    }
}

pub(super) struct Command(bool, Range);
impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("on ") {
            Ok(Self(true, rest.parse()?))
        } else if let Some(rest) = s.strip_prefix("off ") {
            Ok(Self(false, rest.parse()?))
        } else {
            Err(())
        }
    }
}

pub(super) struct Area {
    ranges: Vec<Range>,
}
impl Default for Area {
    fn default() -> Self {
        Self::new()
    }
}
impl Area {
    pub(super) fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn insert(&mut self, new: Range) {
        // Remove from current range those that are contained by the new range
        self.ranges.retain(|r| !new.contains(r));
        for range in &self.ranges {
            if range.intersects(&new) {
                if new.x.0 < range.x.0 {
                    let (new1, new2) = new.split_x(range.x.0 - 1);
                    self.insert(new1);
                    self.insert(new2);
                } else if new.x.1 > range.x.1 {
                    let (new1, new2) = new.split_x(range.x.1);
                    self.insert(new1);
                    self.insert(new2);
                } else if new.y.0 < range.y.0 {
                    let (new1, new2) = new.split_y(range.y.0 - 1);
                    self.insert(new1);
                    self.insert(new2);
                } else if new.y.1 > range.y.1 {
                    let (new1, new2) = new.split_y(range.y.1);
                    self.insert(new1);
                    self.insert(new2);
                } else if new.z.0 < range.z.0 {
                    let (new1, new2) = new.split_z(range.z.0 - 1);
                    self.insert(new1);
                    self.insert(new2);
                } else if new.z.1 > range.z.1 {
                    let (new1, new2) = new.split_z(range.z.1);
                    self.insert(new1);
                    self.insert(new2);
                } else {
                    // fully contained
                }
                return;
            }
        }
        self.ranges.push(new);
    }

    fn remove(&mut self, new: Range) {
        let (conflicts, keep): (Vec<_>, Vec<_>) = self
            .ranges
            .iter()
            .copied()
            .partition(|range| range.intersects(&new));
        self.ranges = keep;
        for range in conflicts {
            let mut put_back = range;

            if put_back.x.0 < new.x.0 {
                let (put_back1, put_back2) = put_back.split_x(new.x.0 - 1);
                self.ranges.push(put_back1);
                put_back = put_back2;
            }
            if put_back.x.1 > new.x.1 {
                let (put_back1, put_back2) = put_back.split_x(new.x.1);
                self.ranges.push(put_back2);
                put_back = put_back1;
            }
            if put_back.y.0 < new.y.0 {
                let (put_back1, put_back2) = put_back.split_y(new.y.0 - 1);
                self.ranges.push(put_back1);
                put_back = put_back2;
            }
            if put_back.y.1 > new.y.1 {
                let (put_back1, put_back2) = put_back.split_y(new.y.1);
                self.ranges.push(put_back2);
                put_back = put_back1;
            }
            if put_back.z.0 < new.z.0 {
                let (put_back1, put_back2) = put_back.split_z(new.z.0 - 1);
                self.ranges.push(put_back1);
                put_back = put_back2;
            }
            if put_back.z.1 > new.z.1 {
                let (_put_back1, put_back2) = put_back.split_z(new.z.1);
                self.ranges.push(put_back2);
                // put_back = put_back1;
            }
        }
    }

    pub(super) fn area(&self) -> usize {
        self.ranges
            .iter()
            .fold(0usize, |acc, range| acc + range.size())
    }

    pub(super) fn step(&mut self, command: Command) {
        if command.0 {
            self.insert(command.1)
        } else {
            self.remove(command.1)
        }
    }
}

pub fn run(input: &str) -> usize {
    let commands: Vec<Command> = input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .filter_map(|Command(neg, range)| {
            if range.x.1 < -50
                || range.x.0 > 50
                || range.y.1 < -50
                || range.y.0 > 50
                || range.z.1 < -50
                || range.z.0 > 50
            {
                None
            } else {
                let new_range = Range {
                    x: (range.x.0.max(-50), range.x.1.min(50)),
                    y: (range.y.0.max(-50), range.y.1.min(50)),
                    z: (range.z.0.max(-50), range.z.1.min(50)),
                };
                Some(Command(neg, new_range))
            }
        })
        .collect();
    let mut area = Area::new();
    for command in commands {
        area.step(command);
    }
    area.area()
}
