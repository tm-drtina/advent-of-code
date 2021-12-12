use std::collections::VecDeque;

fn fill(row: usize, col: usize, map: &mut Vec<Vec<u32>>) -> usize {
    let mut q = VecDeque::new();
    q.push_back((row, col));
    let mut count = 0;
    while !q.is_empty() {
        let (row, col) = q.pop_front().unwrap();
        if map[row][col] == 9 {
            continue;
        }
        if row > 0 && map[row - 1][col] < 9 && map[row][col] < map[row - 1][col] {
            q.push_back((row - 1, col));
        }
        if row < map.len() - 1 && map[row + 1][col] < 9 && map[row][col] < map[row + 1][col] {
            q.push_back((row + 1, col));
        }
        if col > 0 && map[row][col - 1] < 9 && map[row][col] < map[row][col - 1] {
            q.push_back((row, col - 1));
        }
        if col < map[0].len() - 1 && map[row][col + 1] < 9 && map[row][col] < map[row][col + 1] {
            q.push_back((row, col + 1));
        }
        map[row][col] = 9;
        count += 1;
    }
    count
}

pub fn run(input: &str) -> usize {
    let mut map: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().map(|ch| ch as u32 - '0' as u32).collect())
        .collect();
    let rows = map.len();
    let cols = map[0].len();
    let mut vals: Vec<usize> = (0..rows)
        .flat_map(|row| {
            (0..cols)
                .filter_map(|col| {
                    let top = row == 0 || map[row - 1][col] > map[row][col];
                    let bot = row == map.len() - 1 || map[row + 1][col] > map[row][col];
                    let left = col == 0 || map[row][col - 1] > map[row][col];
                    let right = col == map[0].len() - 1 || map[row][col + 1] > map[row][col];
                    if top && bot && left && right {
                        Some(fill(row, col, &mut map))
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect();
    vals.sort_unstable();
    vals.reverse();
    vals[0] * vals[1] * vals[2]
}
