pub(super) fn step(map: &mut Vec<Vec<u8>>) -> usize {
    let mut q = Vec::new();
    let mut flashes = 0;
    for x in 0..10 {
        for y in 0..10 {
            map[x][y] += 1;
            if map[x][y] > 9 {
                q.push((x, y));
                flashes += 1;
            }
        }
    }

    while !q.is_empty() {
        let (x1, y1) = q.pop().unwrap();
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
    for x in 0..10 {
        for y in 0..10 {
            if map[x][y] > 9 {
                map[x][y] = 0;
            }
        }
    }
    flashes
}

pub(super) fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch as u8 - '0' as u8).collect())
        .collect()
}

pub fn run(input: &str) -> usize {
    let mut map = parse(input);
    (0..100).fold(0, |acc, _| acc + step(&mut map))
}
