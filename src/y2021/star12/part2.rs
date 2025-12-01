use std::collections::{HashMap, HashSet};

use super::part1::{Node, parse};

fn find_paths(
    visited: &HashSet<Node>,
    visited_twice: bool,
    current: Node,
    lines: &HashMap<Node, Vec<Node>>,
) -> usize {
    lines
        .get(&current)
        .unwrap() // We got there somehow, so there is way back
        .iter()
        .copied()
        .map(|next| match next {
            Node::Start => 0,
            Node::End => 1,
            Node::Big(_) => find_paths(visited, visited_twice, next, lines),
            Node::Small(_) if visited_twice && visited.contains(&next) => 0,
            Node::Small(_) => {
                let mut visited = visited.clone();
                let visited_twice = visited_twice || visited.contains(&next);
                visited.insert(next);
                find_paths(&visited, visited_twice, next, lines)
            }
        })
        .sum()
}

pub fn run(input: &str) -> usize {
    let lines = parse(input);
    find_paths(&HashSet::new(), false, Node::Start, &lines)
}
