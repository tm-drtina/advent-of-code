use std::str::FromStr;

use anyhow::Result;

use crate::utils::map::Map;
use crate::utils::point::Point2D;

struct State {
    position: Point2D<usize>,
    next: Point2D<usize>,
    map: Map<(char, bool)>,
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let mut map = Map::from_str(s, |ch, x, y| {
            if ch == 'S' {
                start = Some(Point2D { x, y });
            }
            (ch, false)
        });
        let start = start.unwrap();
        let mut start_neighbors = map.four_neighborhood(start).into_iter().filter(|n| {
            let n = *n;
            let n_symbol = map.at(n).0;
            (start.right() == n && ['-', 'J', '7'].contains(&n_symbol))
                || (start.bottom() == n && ['|', 'L', 'J'].contains(&n_symbol))
                || (start.try_left() == Some(n) && ['-', 'L', 'F'].contains(&n_symbol))
                || (start.try_top() == Some(n) && ['|', '7', 'F'].contains(&n_symbol))
        });

        let n1 = start_neighbors.next().unwrap();
        let n2 = start_neighbors.next().unwrap();

        let start_char = if n1.x == n2.x {
            '|'
        } else if n1.y == n2.y {
            '-'
        } else if n1.x > start.x || n2.x > start.x {
            if n1.y > start.y || n2.y > start.y {
                'F'
            } else {
                'L'
            }
        } else if n1.x < start.x || n2.x < start.x {
            if n1.y > start.y || n2.y > start.y {
                '7'
            } else {
                'J'
            }
        } else {
            unreachable!()
        };

        map.set(start, (start_char, true));

        Ok(Self {
            position: start,
            next: n1,
            map,
        })
    }
}

impl Iterator for State {
    type Item = Point2D<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.map
            .set(self.position, (self.map.at(self.position).0, true));
        if self.map.at(self.next).1 {
            return None;
        }

        let next = match self.map.at(self.next).0 {
            '|' if self.next.y == self.position.bottom().y => self.next.bottom(),
            '|' if self.next.y == self.position.try_top().unwrap().y => {
                self.next.try_top().unwrap()
            }
            '-' if self.next.x == self.position.right().x => self.next.right(),
            '-' if self.next.x == self.position.try_left().unwrap().x => {
                self.next.try_left().unwrap()
            }
            'L' if self.next.x == self.position.x => self.next.right(),
            'L' if self.next.y == self.position.y => self.next.try_top().unwrap(),
            'J' if self.next.x == self.position.x => self.next.try_left().unwrap(),
            'J' if self.next.y == self.position.y => self.next.try_top().unwrap(),
            '7' if self.next.x == self.position.x => self.next.try_left().unwrap(),
            '7' if self.next.y == self.position.y => self.next.bottom(),
            'F' if self.next.x == self.position.x => self.next.right(),
            'F' if self.next.y == self.position.y => self.next.bottom(),
            ch => unreachable!("Invalid next point {ch}"),
        };
        self.position = self.next;
        self.next = next;
        Some(self.position)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Loc {
    NoWall,
    InWallFromTop,
    InWallFromBottom,
}

pub fn run(input: &str) -> Result<usize> {
    let mut s: State = input.parse()?;
    for _ in s.by_ref() {}
    Ok(s.map
        .iter_values()
        .scan(
            (false, Loc::NoWall),
            |(inside, pos), (point, &(ch, is_wall))| {
                if point.x == 0 {
                    *inside = false;
                    *pos = Loc::NoWall;
                }
                match (ch, is_wall) {
                    ('|', true) => *inside = !*inside,
                    ('-', true) => {}
                    ('L', true) => *pos = Loc::InWallFromTop,
                    ('F', true) => *pos = Loc::InWallFromBottom,
                    ('J', true) if *pos == Loc::InWallFromBottom => *inside = !*inside,
                    ('J', true) if *pos == Loc::InWallFromTop => {}
                    ('7', true) if *pos == Loc::InWallFromTop => *inside = !*inside,
                    ('7', true) if *pos == Loc::InWallFromBottom => {}
                    (_, false) => {
                        if *inside {
                            return Some(1);
                        }
                    }
                    _ => unreachable!("{ch}: {is_wall}"),
                }
                Some(0)
            },
        )
        .sum())
}
