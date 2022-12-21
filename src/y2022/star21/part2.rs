use anyhow::{anyhow, Result};

use super::common::{Node, Puzzle, Value};

pub fn run(input: &str) -> Result<Value> {
    let mut puzzle: Puzzle = input.parse()?;
    let (l, r) = puzzle.get_root_subtrees();
    puzzle.mark_humn_as_var();
    let left = puzzle.flatten(l);
    let right = puzzle.flatten(r);
    match (left, right) {
        (Node::Literal(lit), node) | (node, Node::Literal(lit)) => {
            puzzle.resolve_for_variable(lit, node)
        }
        _ => Err(anyhow!("At least one subtree must be literal")),
    }
}
