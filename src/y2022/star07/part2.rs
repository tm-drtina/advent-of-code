use anyhow::{anyhow, Result};

use super::common::Node;

fn find(node: &Node, min_size: usize) -> Option<usize> {
    match node {
        Node::File { .. } => None,
        Node::Dir { size, childs, .. } => {
            if *size < min_size {
                None
            } else {
                childs
                    .values()
                    .filter_map(|n| find(n, min_size))
                    .min()
                    .or(Some(*size))
            }
        }
    }
}

pub fn run(input: &'static str) -> Result<usize> {
    let mut lines = input.lines();

    // skip jump to root dir - always there and on first line
    let first_line = lines.next();
    debug_assert_eq!(first_line, Some("$ cd /"));

    let mut root = Node::root();
    root.load(&mut lines)?;
    let root_size = match &root {
        Node::File { size, .. } | Node::Dir { size, .. } => *size,
    };

    find(&root, root_size - 40_000_000).ok_or(anyhow!("Not found"))
}
