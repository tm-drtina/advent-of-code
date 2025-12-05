use crate::utils::map::Map;

pub fn run(input: &str) -> usize {
    let mut map = Map::from_str(input, |c, _x, _y| c == '@');
    let mut count = 0;
    loop {
        let to_remove = map
            .iter_values()
            .filter(|(pt, a)| {
                **a && pt
                    .try_eight_neighborhood()
                    .into_iter()
                    .filter(|n| matches!(map.try_at(*n).copied(), Some(true)))
                    .count()
                    < 4
            })
            .map(|(pt, _)| pt)
            .collect::<Vec<_>>();
        if to_remove.is_empty() {
            break count;
        }
        count += to_remove.len();
        for pt in to_remove {
            map.set(pt, false);
        }
    }
}
