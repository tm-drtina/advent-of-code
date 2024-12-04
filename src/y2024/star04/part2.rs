use anyhow::Result;

use crate::utils::map::Map;

pub fn run(input: &str) -> Result<usize> {
    let map = Map::from_str(input, |ch, _x, _y| ch as u8);

    Ok(map
        .iter_values()
        .filter_map(|(pt, ch)| {
            if *ch != b'A' {
                return None;
            }
            let tl = map.try_at(pt.try_top_left()?)?;
            let tr = map.try_at(pt.try_top_right()?)?;
            let bl = map.try_at(pt.try_bottom_left()?)?;
            let br = map.try_at(pt.bottom_right())?;

            if !matches!((tl, br), (b'M', b'S') | (b'S', b'M')) {
                return None;
            }
            if !matches!((tr, bl), (b'M', b'S') | (b'S', b'M')) {
                return None;
            }
            Some(())
        })
        .count())
}
