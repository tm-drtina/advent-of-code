use std::collections::BTreeSet;

use anyhow::{Result, anyhow};

use crate::utils::map::Map;
use crate::utils::point::{Dir, Point2D};

#[derive(Debug, Clone)]
enum Tile {
    Blocked,
    Empty,
    Visited(BTreeSet<Dir>),
    Guard(Dir),
}

fn in_loop(mut map: Map<Tile>, mut pos: Point2D<usize>, mut dir: Dir) -> bool {
    loop {
        if let Tile::Visited(dirs) = map.at_mut(pos) {
            dirs.insert(dir);
        } else {
            let mut dirs = BTreeSet::new();
            dirs.insert(dir);
            map.set(pos, Tile::Visited(dirs));
        }

        let Some(new_pos) = pos.try_step_dir(dir) else {
            return false;
        };
        if map.try_at(new_pos).is_none() {
            return false;
        }

        if matches!(map.at(new_pos), Tile::Blocked) {
            dir = dir.clockwise_90();
        } else {
            pos = new_pos;
            match map.at(new_pos) {
                Tile::Blocked | Tile::Guard(_) => unreachable!(),
                Tile::Visited(dirs) if dirs.contains(&dir) => return true,
                _ => {}
            }
        }
    }
}

pub fn run(input: &str) -> Result<usize> {
    let map = Map::from_str(input, |ch, _x, _y| match ch as u8 {
        b'.' => Tile::Empty,
        b'#' => Tile::Blocked,
        b'^' => Tile::Guard(Dir::Top),
        _ => unreachable!(),
    });

    let (pos, tile) = map
        .iter_values()
        .find(|(_pt, tile)| matches!(tile, Tile::Guard(_)))
        .ok_or(anyhow!("Guard position not found"))?;
    let Tile::Guard(dir) = *tile else {
        unreachable!()
    };

    Ok(map
        .iter_values()
        .filter(|(pt, tile)| {
            if !matches!(tile, Tile::Empty) {
                return false;
            }
            let mut clone = map.clone();
            clone.set(*pt, Tile::Blocked);
            in_loop(clone, pos, dir)
        })
        .count())
}
