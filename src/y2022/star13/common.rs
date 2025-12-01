use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub(super) enum Packet {
    Literal(u32),
    Nested(Vec<Packet>),
}

impl From<&mut &[u8]> for Packet {
    fn from(value: &mut &[u8]) -> Self {
        match value[0] {
            b'[' => {
                if value[1] == b']' {
                    *value = &value[2..];
                    return Self::Nested(Vec::new());
                }
                *value = &value[1..];
                let mut res = Vec::new();
                loop {
                    res.push(Packet::from(&mut *value));
                    match value[0] {
                        b',' => {
                            *value = &value[1..];
                        }
                        b']' => {
                            *value = &value[1..];
                            return Self::Nested(res);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            b'0'..=b'9' => {
                let mut num = 0;
                while value[0] >= b'0' && value[0] <= b'9' {
                    num = num * 10 + (value[0] - b'0') as u32;
                    *value = &value[1..];
                }
                Self::Literal(num)
            }
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Literal(lhs), Packet::Literal(rhs)) => lhs.cmp(rhs),
            (Packet::Nested(lhs), Packet::Nested(rhs)) => {
                for (l, r) in lhs.iter().zip(rhs) {
                    match l.cmp(r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {},
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                lhs.len().cmp(&rhs.len())
            }
            (Packet::Literal(lhs), rhs @ Packet::Nested(_)) => {
                Packet::Nested(vec![Packet::Literal(*lhs)]).cmp(rhs)
            }
            (lhs @ Packet::Nested(_), Packet::Literal(rhs)) => {
                lhs.cmp(&Packet::Nested(vec![Packet::Literal(*rhs)]))
            }
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Literal(val) => write!(f, "{val}"),
            Packet::Nested(val) => {
                write!(f, "[")?;
                for (index, p) in val.iter().enumerate() {
                    if index > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{p}")?;
                }
                write!(f, "]")
            }
        }
    }
}
