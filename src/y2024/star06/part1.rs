use anyhow::{Result, anyhow};

use crate::utils::map::Map;
use crate::utils::point::Dir;

enum Tile {
    Blocked,
    Empty,
    Visited,
    Guard(Dir),
}

pub fn run(input: &str) -> Result<usize> {
    let mut map = Map::from_str(input, |ch, _x, _y| match ch as u8 {
        b'.' => Tile::Empty,
        b'#' => Tile::Blocked,
        b'^' => Tile::Guard(Dir::Top),
        _ => unreachable!(),
    });

    let (mut pos, tile) = map
        .iter_values()
        .find(|(_pt, tile)| matches!(tile, Tile::Guard(_)))
        .ok_or(anyhow!("Guard position not found"))?;
    let &Tile::Guard(mut dir) = tile else {
        unreachable!()
    };
    loop {
        map.set(pos, Tile::Visited);
        let Some(new_pos) = pos.try_step_dir(dir) else {
            break;
        };
        if map.try_at(new_pos).is_none() {
            break;
        }
        if matches!(map.at(new_pos), Tile::Blocked) {
            dir = dir.clockwise_90();
        } else {
            pos = new_pos;
        }
    }

    Ok(map
        .iter_values()
        .filter(|(_pt, tile)| matches!(tile, Tile::Visited))
        .count())
}
