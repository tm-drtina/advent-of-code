pub(super) fn step(map: &mut Vec<Vec<u8>>) -> usize {
    let mut q = Vec::new();
    let mut flashes = 0;
    for (x, row) in map.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            *cell += 1;
            if *cell > 9 {
                q.push((x, y));
                flashes += 1;
            }
        }
    }

    while let Some((x1, y1)) = q.pop() {
        #[allow(clippy::needless_range_loop)]
        for x in (x1.max(1) - 1)..=(x1.min(8) + 1) {
            for y in (y1.max(1) - 1)..=(y1.min(8) + 1) {
                if x == x1 && y == y1 {
                    continue;
                }
                map[x][y] += 1;
                if map[x][y] == 10 {
                    q.push((x, y));
                    flashes += 1;
                }
            }
        }
    }

    for row in map {
        for cell in row {
            if *cell > 9 {
                *cell = 0;
            }
        }
    }
    flashes
}

pub(super) fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect())
        .collect()
}

pub fn run(input: &str) -> usize {
    let mut map = parse(input);
    (0..100).fold(0, |acc, _| acc + step(&mut map))
}
