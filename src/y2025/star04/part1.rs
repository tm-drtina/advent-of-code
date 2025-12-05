use crate::utils::map::Map;

pub fn run(input: &str) -> usize {
    let map = Map::from_str(input, |c, _x, _y| c == '@');
    map.iter_values()
        .filter(|(pt, a)| {
            **a && pt
                .try_eight_neighborhood()
                .into_iter()
                .filter(|n| matches!(map.try_at(*n).copied(), Some(true)))
                .count()
                < 4
        })
        .count()
}
