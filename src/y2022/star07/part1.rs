use anyhow::Result;

use super::common::Node;

fn sizes(node: &Node) -> usize {
    match node {
        Node::File { .. } => 0,
        Node::Dir { size, childs, .. } => {
            let c = childs.values().map(sizes).sum();
            let size = *size;
            if size < 100_000 { size + c } else { c }
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

    Ok(sizes(&root))
}
