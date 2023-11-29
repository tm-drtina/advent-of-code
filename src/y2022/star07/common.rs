use std::collections::HashMap;
use std::str::Lines;

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub(super) enum Node {
    File {
        name: &'static str,
        size: usize,
    },
    Dir {
        name: &'static str,
        size: usize,
        childs: HashMap<&'static str, Node>,
    },
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(0, f)
    }
}

impl Node {
    pub(super) fn root() -> Self {
        Self::Dir {
            name: "<root>",
            size: 0,
            childs: HashMap::default(),
        }
    }

    pub(super) fn load(&mut self, lines: &mut Lines<'static>) -> Result<usize> {
        let Node::Dir {
            name: _,
            size,
            childs,
        } = self
        else {
            unreachable!("Cannot load files!")
        };
        loop {
            let Some(line) = lines.next() else {
                return Ok(*size);
            };
            match line {
                "$ ls" => {}
                "$ cd .." => {
                    return Ok(*size);
                }
                _ if line.starts_with("$ cd ") => {
                    let dir = &line[5..];
                    *size += childs
                        .get_mut(dir)
                        .ok_or_else(|| anyhow!("Dir not found"))?
                        .load(lines)?;
                }
                _ if line.starts_with("dir ") => {
                    let name = &line[4..];
                    childs.insert(
                        name,
                        Node::Dir {
                            name,
                            size: 0,
                            childs: HashMap::default(),
                        },
                    );
                }
                _ => {
                    let (fsize, name) = line
                        .split_once(' ')
                        .ok_or_else(|| anyhow!("Invalid file format"))?;
                    let fsize = fsize.parse::<usize>()?;
                    childs.insert(name, Node::File { name, size: fsize });
                    *size += fsize;
                }
            }
        }
    }

    fn pretty_print(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", " ".repeat(indent))?;
        match self {
            Node::File { name, size } => {
                writeln!(f, "- {name} (file, size={size})")
            }
            Node::Dir { name, size, childs } => {
                writeln!(f, "- {name} (dir, size={size})")?;
                for child in childs.values() {
                    child.pretty_print(indent + 2, f)?;
                }
                Ok(())
            }
        }
    }
}
