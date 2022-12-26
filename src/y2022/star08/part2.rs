use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<usize> {
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();
    (1..map.len() - 1)
        .flat_map(|i| {
            let map = &map;
            (1..map[i].len() - 1).map(move |j| {
                let l = (0..i)
                    .rev()
                    .position(|i2| map[i2][j] >= map[i][j])
                    .map_or(i, |x| x + 1);
                let r = (i + 1..map.len())
                    .position(|i2| map[i2][j] >= map[i][j])
                    .map_or(map.len() - i - 1, |x| x + 1);
                let t = (0..j)
                    .rev()
                    .position(|j2| map[i][j2] >= map[i][j])
                    .map_or(j, |x| x + 1);
                let b = (j + 1..map[i].len())
                    .position(|j2| map[i][j2] >= map[i][j])
                    .map_or(map[i].len() - j - 1, |x| x + 1);
                l * r * t * b
            })
        })
        .max()
        .ok_or_else(|| anyhow!("No trees found"))
}
