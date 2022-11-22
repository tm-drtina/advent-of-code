use std::cmp::Ordering;

use itertools::Itertools;

pub trait SortedByAngle<T>
where
    Self: Iterator<Item = T>,
{
    fn sorted_by_angle<F, V>(&mut self, f: F, min_angle: f64) -> std::vec::IntoIter<T>
    where
        F: Fn(&T) -> (V, V),
        f64: From<V>,
    {
        self.sorted_by(|a, b| {
            let (dx1, dy1) = f(a);
            let (dx2, dy2) = f(b);
            let mut angle1 = f64::from(dy1).atan2(f64::from(dx1)) + min_angle;
            let mut angle2 = f64::from(dy2).atan2(f64::from(dx2)) + min_angle;

            while angle1 <= -std::f64::consts::PI {
                angle1 += 2_f64 * std::f64::consts::PI
            }
            while angle1 > std::f64::consts::PI {
                angle1 -= 2_f64 * std::f64::consts::PI
            }
            while angle2 <= -std::f64::consts::PI {
                angle2 += 2_f64 * std::f64::consts::PI
            }
            while angle2 > std::f64::consts::PI {
                angle2 -= 2_f64 * std::f64::consts::PI
            }

            if angle2.lt(&angle1) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

impl<T, I: Iterator<Item = T>> SortedByAngle<T> for I {}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::SortedByAngle;

    #[derive(Debug, Eq, PartialEq)]
    struct TestI32 {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct TestF64 {
        id: i32,
        x: f64,
        y: f64,
    }

    impl PartialEq for TestF64 {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[test]
    fn test_i32() {
        let expected = vec![
            TestI32 { x: 0, y: 1 },
            TestI32 { x: 1, y: 1 },
            TestI32 { x: 1, y: 0 },
            TestI32 { x: 1, y: -1 },
            TestI32 { x: 0, y: -1 },
            TestI32 { x: -1, y: -1 },
            TestI32 { x: -1, y: 0 },
            TestI32 { x: -1, y: 1 },
        ];
        let actual = vec![
            TestI32 { x: -1, y: 0 },
            TestI32 { x: 0, y: -1 },
            TestI32 { x: 1, y: 1 },
            TestI32 { x: 1, y: -1 },
            TestI32 { x: -1, y: -1 },
            TestI32 { x: 0, y: 1 },
            TestI32 { x: 1, y: 0 },
            TestI32 { x: -1, y: 1 },
        ]
        .into_iter()
        .sorted_by_angle(|foo| (foo.x, foo.y), std::f64::consts::FRAC_PI_2)
        .collect_vec();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_f64() {
        let expected = vec![
            TestF64 {
                id: 1,
                x: 0.0,
                y: 1.0,
            },
            TestF64 {
                id: 2,
                x: 0.01,
                y: 1.0,
            },
            TestF64 {
                id: 3,
                x: 0.01,
                y: 1.0,
            },
            TestF64 {
                id: 4,
                x: 1.0,
                y: 1.0,
            },
            TestF64 {
                id: 5,
                x: 1.0,
                y: 0.0,
            },
            TestF64 {
                id: 6,
                x: 1.0,
                y: -1.0,
            },
            TestF64 {
                id: 7,
                x: 0.0,
                y: -1.0,
            },
            TestF64 {
                id: 8,
                x: -1.0,
                y: -1.0,
            },
            TestF64 {
                id: 9,
                x: -1.0,
                y: 0.0,
            },
            TestF64 {
                id: 10,
                x: -1.0,
                y: 1.0,
            },
            TestF64 {
                id: 11,
                x: -0.01,
                y: 1.0,
            },
        ];
        let actual = vec![
            TestF64 {
                id: 2,
                x: 0.01,
                y: 1.0,
            },
            TestF64 {
                id: 5,
                x: 1.0,
                y: 0.0,
            },
            TestF64 {
                id: 1,
                x: 0.0,
                y: 1.0,
            },
            TestF64 {
                id: 3,
                x: 0.01,
                y: 1.0,
            },
            TestF64 {
                id: 4,
                x: 1.0,
                y: 1.0,
            },
            TestF64 {
                id: 6,
                x: 1.0,
                y: -1.0,
            },
            TestF64 {
                id: 9,
                x: -1.0,
                y: 0.0,
            },
            TestF64 {
                id: 11,
                x: -0.01,
                y: 1.0,
            },
            TestF64 {
                id: 7,
                x: 0.0,
                y: -1.0,
            },
            TestF64 {
                id: 10,
                x: -1.0,
                y: 1.0,
            },
            TestF64 {
                id: 8,
                x: -1.0,
                y: -1.0,
            },
        ]
        .into_iter()
        .sorted_by_angle(|foo| (foo.x, foo.y), std::f64::consts::FRAC_PI_2)
        .collect_vec();

        assert_eq!(expected, actual);
    }
}
