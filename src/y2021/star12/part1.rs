use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(super) enum Node {
    Start,
    End,
    Small(u16),
    Big(u16),
}

/// Nodes are always 2 chars/bytes, so it's safe to keep them as u16
fn str_to_int(s: &str) -> u16 {
    s.as_bytes()
        .iter()
        .fold(0, |acc, val| (acc << 8) + *val as u16)
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            _ if s.chars().all(|ch| ch.is_ascii_lowercase()) => Ok(Self::Small(str_to_int(s))),
            _ if s.chars().all(|ch| ch.is_ascii_uppercase()) => Ok(Self::Big(str_to_int(s))),
            _ => Err(()),
        }
    }
}

pub(super) fn parse(input: &str) -> HashMap<Node, Vec<Node>> {
    let mut lines: HashMap<Node, Vec<Node>> = HashMap::new();

    for line in input.lines() {
        let (node1, node2) = line.split_once('-').unwrap();
        let node1 = node1.parse().unwrap();
        let node2 = node2.parse().unwrap();
        lines.entry(node1).or_default().push(node2);
        lines.entry(node2).or_default().push(node1);
    }

    lines
}

fn find_paths(visited: &HashSet<Node>, current: Node, lines: &HashMap<Node, Vec<Node>>) -> usize {
    lines
        .get(&current)
        .unwrap_or(&Vec::new())
        .iter()
        .copied()
        .map(|next| match next {
            Node::Start => 0,
            Node::End => 1,
            Node::Big(_) => find_paths(visited, next, lines),
            Node::Small(_) if visited.contains(&next) => 0,
            Node::Small(_) => {
                let mut visited = visited.clone();
                visited.insert(next);
                find_paths(&visited, next, lines)
            }
        })
        .sum()
}

pub fn run(input: &str) -> usize {
    let lines = parse(input);
    find_paths(&HashSet::new(), Node::Start, &lines)
}
