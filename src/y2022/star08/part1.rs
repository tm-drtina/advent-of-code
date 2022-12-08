pub fn run(input: &str) -> usize {
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();
    (1..map.len() - 1)
        .flat_map(|i| {
            let map = &map;
            (1..map[i].len() - 1).map(move |j| {
                let l = (0..i).all(|i2| map[i2][j] < map[i][j]);
                let r = (i + 1..map.len()).all(|i2| map[i2][j] < map[i][j]);
                let t = (0..j).all(|j2| map[i][j2] < map[i][j]);
                let b = (j + 1..map[i].len()).all(|j2| map[i][j2] < map[i][j]);
                if l || r || t || b { 1 } else { 0 }
            })
        })
        .sum::<usize>()
        + map.len() * 2
        + map[0].len() * 2
        - 4
}
