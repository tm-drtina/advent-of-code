use std::str::Chars;

#[derive(Debug, Clone)]
pub(super) enum Number {
    Literal(usize),
    Combined(Box<Number>, Box<Number>),
}

impl Number {
    pub(super) fn from_str(chars: &mut Chars) -> Self {
        let ch = chars.next().unwrap();
        if ch == '[' {
            let left = Box::new(Self::from_str(chars));
            let right = Box::new(Self::from_str(chars));
            chars.next();
            Self::Combined(left, right)
        } else {
            chars.next();
            Self::Literal((ch as u8 - b'0') as usize)
        }
    }

    pub(super) fn add(self, other: Self) -> Self {
        Self::Combined(Box::new(self), Box::new(other))
    }

    fn increment_left(&mut self, amount: usize) {
        match self {
            Number::Literal(value) => {
                *value += amount;
            }
            Number::Combined(left, _) => {
                left.increment_left(amount);
            }
        }
    }

    fn increment_right(&mut self, amount: usize) {
        match self {
            Number::Literal(value) => {
                *value += amount;
            }
            Number::Combined(_, right) => {
                right.increment_right(amount);
            }
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<usize>, Option<usize>) {
        match self {
            Number::Literal(_) => (false, None, None),
            Number::Combined(left, right) => {
                if depth == 3 {
                    if let Number::Combined(inner_left, inner_right) = &**left {
                        let left_value = if let Number::Literal(value) = **inner_left {
                            value
                        } else {
                            unreachable!("Well, this is awkward")
                        };
                        let right_value = if let Number::Literal(value) = **inner_right {
                            value
                        } else {
                            unreachable!("Well, this is awkward")
                        };

                        *left = Box::new(Self::Literal(0));
                        right.increment_left(right_value);
                        (true, Some(left_value), None)
                    } else if let Number::Combined(inner_left, inner_right) = &**right {
                        let left_value = if let Number::Literal(value) = **inner_left {
                            value
                        } else {
                            unreachable!("Well, this is awkward")
                        };
                        let right_value = if let Number::Literal(value) = **inner_right {
                            value
                        } else {
                            unreachable!("Well, this is awkward")
                        };

                        left.increment_right(left_value);
                        *right = Box::new(Self::Literal(0));
                        (true, None, Some(right_value))
                    } else {
                        (false, None, None)
                    }
                } else {
                    let (res, left_inc, right_inc) = left.explode(depth + 1);
                    if res {
                        if let Some(right_inc) = right_inc {
                            right.increment_left(right_inc);
                        }
                        return (true, left_inc, None);
                    }
                    let (res, left_inc, right_inc) = right.explode(depth + 1);
                    if res {
                        if let Some(left_inc) = left_inc {
                            left.increment_right(left_inc);
                        }
                        (true, None, right_inc)
                    } else {
                        (false, None, None)
                    }
                }
            }
        }
    }

    fn split(&mut self) -> (bool, Option<Number>) {
        match self {
            Number::Literal(val) => {
                if *val > 9 {
                    let left = *val / 2;
                    let right = *val - left;
                    (
                        true,
                        Some(Number::Combined(
                            Box::new(Number::Literal(left)),
                            Box::new(Number::Literal(right)),
                        )),
                    )
                } else {
                    (false, None)
                }
            }
            Number::Combined(left, right) => {
                let (res, replace) = left.split();
                if res {
                    if let Some(replace) = replace {
                        *left = Box::new(replace);
                    }
                    return (true, None);
                }
                let (res, replace) = right.split();
                if res {
                    if let Some(replace) = replace {
                        *right = Box::new(replace);
                    }
                    (true, None)
                } else {
                    (false, None)
                }
            }
        }
    }

    pub(super) fn reduce(&mut self) {
        loop {
            if self.explode(0).0 {
                continue;
            }
            if self.split().0 {
                continue;
            }
            break;
        }
    }

    pub(super) fn magnitude(&self) -> usize {
        match self {
            Number::Literal(value) => *value as usize,
            Number::Combined(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Literal(val) => f.write_fmt(format_args!("{}", val)),
            Number::Combined(l, r) => f.write_fmt(format_args!("[{},{}]", l, r)),
        }
    }
}

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| Number::from_str(&mut line.chars()))
        .reduce(|acc, val| {
            let mut res = acc.add(val);
            res.reduce();
            res
        })
        .unwrap()
        .magnitude()
}
