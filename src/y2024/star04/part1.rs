use anyhow::Result;

use crate::utils::map::Map;
use crate::utils::point::Dir;

pub fn run(input: &str) -> Result<usize> {
    let map = Map::from_str(input, |ch, _x, _y| ch as u8);

    Ok(map
        .iter_values()
        .map(|(pt, _ch)| {
            Dir::iter()
                .filter_map(|&dir| {
                    let step1 = pt.try_step_dir(dir)?;
                    let step2 = step1.try_step_dir(dir)?;
                    let step3 = step2.try_step_dir(dir)?;
                    if map.is_valid_point(&step3) {
                        Some((pt, step1, step2, step3))
                    } else {
                        None
                    }
                })
                .map(|(pt1, pt2, pt3, pt4)| (map.at(pt1), map.at(pt2), map.at(pt3), map.at(pt4)))
                .filter(|chars| matches!(chars, (b'X', b'M', b'A', b'S')))
                .count()
        })
        .sum())
}
