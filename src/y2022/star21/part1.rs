use anyhow::Result;

use super::common::{Node, Puzzle, Value};

pub fn run(input: &str) -> Result<Value> {
    match input
        .parse::<Puzzle>()?
        .flatten(Puzzle::name_to_addr("root"))
    {
        Node::Literal(v) => Ok(v),
        _ => unreachable!(),
    }
}
