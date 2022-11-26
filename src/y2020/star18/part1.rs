use std::str::Chars;

enum Op {
    Add,
    Mul,
}

impl Op {
    fn perform(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }
}

enum Expression {
    Term { val: i64 },
    Complex { parts: Vec<(Op, Expression)> },
}

impl Expression {
    fn result(self) -> i64 {
        match self {
            Expression::Term { val } => val,
            Expression::Complex { parts } => parts
                .into_iter()
                .fold(0, |acc, (op, expr)| op.perform(acc, expr.result())),
        }
    }
}
impl<'a> Expression {
    fn from_with_iter_passing(mut chars: Chars<'a>) -> (Self, Chars<'a>) {
        let mut last_op = Op::Add;
        let mut parts: Vec<(Op, Expression)> = Vec::new();
        loop {
            let next = chars.next();
            if next.is_none() {
                break;
            }
            let next = next.unwrap();
            let expr = match next {
                '(' => {
                    let (ex, ch) = Expression::from_with_iter_passing(chars);
                    chars = ch;
                    ex
                }
                '0'..='9' => Expression::Term {
                    val: next as i64 - '0' as i64,
                },
                _ => panic!("Unexpected char '{}'", next),
            };
            parts.push((last_op, expr));

            match chars.next() {
                None | Some(')') => break,
                Some(' ') => {}
                Some(_) => panic!(),
            }
            last_op = match chars.next() {
                Some('+') => Op::Add,
                Some('*') => Op::Mul,
                _ => panic!(),
            };
            chars.next();
        }
        (Expression::Complex { parts }, chars)
    }
}

impl<'a> From<Chars<'a>> for Expression {
    fn from(chars: Chars<'a>) -> Self {
        Expression::from_with_iter_passing(chars).0
    }
}

pub fn run(input: &str) -> i64 {
    input
        .lines()
        .map(|line| Expression::from(line.chars()))
        .map(Expression::result)
        .sum()
}
