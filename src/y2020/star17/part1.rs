use std::cmp::{max, min};
use std::collections::HashSet;

fn active_neighbors(map: &HashSet<(i32, i32, i32)>, x: i32, y: i32, z: i32) -> i32 {
    let mut res = 0;
    for x1 in x - 1..x + 2 {
        for y1 in y - 1..y + 2 {
            for z1 in z - 1..z + 2 {
                if x1 == x && y1 == y && z1 == z {
                    continue;
                }
                if map.contains(&(x1, y1, z1)) {
                    res += 1;
                }
            }
        }
    }
    res
}

pub fn run(input: &str) -> usize {
    let mut min_x = 0_i32;
    let mut min_y = 0_i32;
    let mut min_z = 0_i32;
    let mut max_x = 0_i32;
    let mut max_y = 0_i32;
    let mut max_z = 0_i32;
    let mut map: HashSet<(i32, i32, i32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((x as i32, y as i32, 0));
                max_x = max(max_x, x as i32);
                max_y = max(max_y, y as i32);
            }
        }
    }

    for _cycle in 0..6 {
        let mut new_min_x = min_x;
        let mut new_min_y = min_y;
        let mut new_min_z = min_z;
        let mut new_max_x = max_x;
        let mut new_max_y = max_y;
        let mut new_max_z = max_z;
        let mut new_map: HashSet<(i32, i32, i32)> = HashSet::new();

        for x in min_x - 1..max_x + 2 {
            for y in min_y - 1..max_y + 2 {
                for z in min_z - 1..max_z + 2 {
                    let active_neighbors = active_neighbors(&map, x, y, z);
                    if map.contains(&(x, y, z)) {
                        if active_neighbors == 2 || active_neighbors == 3 {
                            new_map.insert((x, y, z));

                            new_max_x = max(new_max_x, x);
                            new_max_y = max(new_max_y, y);
                            new_max_z = max(new_max_z, z);
                            new_min_x = min(new_min_x, x);
                            new_min_y = min(new_min_y, y);
                            new_min_z = min(new_min_z, z);
                        }
                    } else if active_neighbors == 3 {
                        new_map.insert((x, y, z));
                        new_max_x = max(new_max_x, x);
                        new_max_y = max(new_max_y, y);
                        new_max_z = max(new_max_z, z);
                        new_min_x = min(new_min_x, x);
                        new_min_y = min(new_min_y, y);
                        new_min_z = min(new_min_z, z);
                    }
                }
            }
        }

        min_x = new_min_x;
        min_y = new_min_y;
        min_z = new_min_z;
        max_x = new_max_x;
        max_y = new_max_y;
        max_z = new_max_z;
        map = new_map;
    }

    map.len()
}
