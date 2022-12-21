use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

type Addr = u32;
pub(super) type Value = i64;

#[derive(Debug, Clone, Copy)]
pub(super) enum Node {
    Literal(Value),
    Variable,
    Add(Addr, Addr),
    Sub(Addr, Addr),
    Mul(Addr, Addr),
    Div(Addr, Addr),
}

#[derive(Debug, Clone)]
pub(super) struct Puzzle(HashMap<Addr, Node>);

impl Puzzle {
    pub(super) fn mark_humn_as_var(&mut self) {
        let humn = Puzzle::name_to_addr("humn");
        self.0.insert(humn, Node::Variable);
    }

    pub(super) fn get_root_subtrees(&self) -> (Addr, Addr) {
        match self.0[&Puzzle::name_to_addr("root")] {
            Node::Literal(_) | Node::Variable => unreachable!(),
            Node::Add(l, r) | Node::Sub(l, r) | Node::Mul(l, r) | Node::Div(l, r) => (l, r),
        }
    }

    pub(super) fn resolve_for_variable(&mut self, lit: Value, node: Node) -> Result<Value> {
        let (left, right) = match node {
            Node::Literal(_) => unreachable!(),
            Node::Variable => return Ok(lit),
            Node::Add(l, r) | Node::Sub(l, r) | Node::Mul(l, r) | Node::Div(l, r) => (l, r),
        };

        match (self.0[&left], self.0[&right]) {
            (Node::Literal(nested_lit), nested_node) => match node {
                Node::Literal(_) | Node::Variable => unreachable!(),
                Node::Add(_, _) => self.resolve_for_variable(lit - nested_lit, nested_node),
                Node::Sub(_, _) => self.resolve_for_variable(nested_lit - lit, nested_node),
                Node::Mul(_, _) => {
                    debug_assert_ne!(nested_lit, 0, "{nested_lit} cannot be zero");
                    debug_assert_eq!(
                        lit % nested_lit,
                        0,
                        "{lit} must be divisible by {nested_lit}"
                    );
                    self.resolve_for_variable(lit / nested_lit, nested_node)
                }
                Node::Div(_, _) => {
                    debug_assert_ne!(lit, 0, "{lit} cannot be zero");
                    debug_assert_eq!(
                        nested_lit % lit,
                        0,
                        "{nested_lit} must be divisible by {lit}"
                    );
                    self.resolve_for_variable(nested_lit / lit, nested_node)
                }
            },
            (nested_node, Node::Literal(nested_lit)) => match node {
                Node::Literal(_) | Node::Variable => unreachable!(),
                Node::Add(_, _) => self.resolve_for_variable(lit - nested_lit, nested_node),
                Node::Sub(_, _) => self.resolve_for_variable(lit + nested_lit, nested_node),
                Node::Mul(_, _) => {
                    debug_assert_ne!(nested_lit, 0, "{nested_lit} cannot be zero");
                    debug_assert_eq!(
                        lit % nested_lit,
                        0,
                        "{lit} must be divisible by {nested_lit}"
                    );
                    self.resolve_for_variable(lit / nested_lit, nested_node)
                }
                Node::Div(_, _) => self.resolve_for_variable(lit * nested_lit, nested_node),
            },
            _ => Err(anyhow!("Unresolved DAG detected!")),
        }
    }

    pub(super) fn flatten(&mut self, addr: Addr) -> Node {
        let (l, r) = match self.0[&addr] {
            lit @ (Node::Literal(_) | Node::Variable) => return lit,
            Node::Add(a1, a2) | Node::Sub(a1, a2) | Node::Mul(a1, a2) | Node::Div(a1, a2) => {
                (self.flatten(a1), self.flatten(a2))
            }
        };
        match (l, r) {
            (Node::Literal(l), Node::Literal(r)) => {
                let value = match self.0[&addr] {
                    Node::Literal(_) | Node::Variable => unreachable!(),
                    Node::Add(_, _) => l + r,
                    Node::Sub(_, _) => l - r,
                    Node::Mul(_, _) => l * r,
                    Node::Div(_, _) => l / r,
                };
                let lit = Node::Literal(value);
                self.0.insert(addr, lit);
                lit
            }
            _ => self.0[&addr],
        }
    }

    pub(super) fn name_to_addr(name: &str) -> Addr {
        let b = name.as_bytes();
        ((b[0] as Addr) << 24) + ((b[1] as Addr) << 16) + ((b[2] as Addr) << 8) + b[3] as Addr
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    let (name, line) = line.split_once(": ").ok_or(anyhow!("Invalid format"))?;
                    let addr = Self::name_to_addr(name);
                    let node = if let Some((l, r)) = line.split_once(" + ") {
                        Node::Add(Self::name_to_addr(l), Self::name_to_addr(r))
                    } else if let Some((l, r)) = line.split_once(" - ") {
                        Node::Sub(Self::name_to_addr(l), Self::name_to_addr(r))
                    } else if let Some((l, r)) = line.split_once(" * ") {
                        Node::Mul(Self::name_to_addr(l), Self::name_to_addr(r))
                    } else if let Some((l, r)) = line.split_once(" / ") {
                        Node::Div(Self::name_to_addr(l), Self::name_to_addr(r))
                    } else {
                        Node::Literal(line.parse()?)
                    };

                    Ok((addr, node))
                })
                .collect::<Result<_>>()?,
        ))
    }
}
