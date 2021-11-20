use super::part1::{find_neighbors, parse_tiles, Neighbors};
use crate::utils::rotate_matrix_cc;
use std::collections::HashMap;

impl Neighbors {
    fn rotate_cc(&mut self) {
        let tmp = self.up;
        self.up = self.right;
        self.right = self.down;
        self.down = self.left;
        self.left = tmp;
        self.data = rotate_matrix_cc(&self.data);
    }
    fn flip_upside_down(&mut self) {
        std::mem::swap(&mut self.up, &mut self.down);
        self.data.reverse();
    }
    fn real_data(&self) -> Vec<Vec<bool>> {
        (&self.data[1..self.data.len() - 1])
            .iter()
            .map(|r| r[1..r.len() - 1].iter().copied().collect())
            .collect()
    }
}

fn create_image(neighbors: &HashMap<i32, Neighbors>) -> Vec<Vec<bool>> {
    let mut top_left = neighbors
        .iter()
        .find(|(_, n)| n.is_corner())
        .unwrap()
        .1
        .clone();
    while top_left.right.is_none() || top_left.down.is_none() {
        top_left.rotate_cc();
    }
    let mut first_row: Vec<Neighbors> = vec![top_left];
    for x in 1.. {
        let prev = &first_row[x - 1];
        let next_id = if let Some(id) = prev.right {
            id
        } else {
            break;
        };
        let mut next = neighbors.get(&next_id).unwrap().clone();
        while next.left != Some(prev.id) {
            next.rotate_cc();
        }
        if next.up.is_some() {
            next.flip_upside_down();
        }
        first_row.push(next);
    }
    let mut image_tmp = vec![first_row];
    loop {
        let top = image_tmp.last().unwrap().first().unwrap();
        if top.down.is_none() {
            break;
        }
        let mut first = neighbors.get(&top.down.unwrap()).unwrap().clone();

        if first.is_corner() {
            while first.right.is_none() || first.up.is_none() {
                first.rotate_cc();
            }
            if first.up != Some(top.id) {
                first.flip_upside_down();
            }
            while first.right.is_none() || first.up.is_none() {
                first.rotate_cc();
            }
        } else {
            while first.left.is_some() {
                first.rotate_cc();
            }
            if first.up != Some(top.id) {
                first.flip_upside_down();
            }
        }

        assert_eq!(first.up, Some(top.id));
        let mut row: Vec<Neighbors> = vec![first];
        for x in 1..image_tmp.first().unwrap().len() {
            let top = &image_tmp.last().unwrap()[x];
            let prev = &row.last().unwrap();
            let next_id = prev.right.unwrap();
            let mut next = neighbors.get(&next_id).unwrap().clone();
            while next.left != Some(prev.id) {
                next.rotate_cc();
            }
            if next.up != Some(top.id) {
                next.flip_upside_down();
            }
            row.push(next);
        }
        image_tmp.push(row);
    }

    image_tmp
        .into_iter()
        .flat_map(|line| {
            line[1..].iter().fold(line[0].real_data(), |v1, n| {
                v1.into_iter()
                    .zip(n.real_data())
                    .map(|(mut r1, r2)| {
                        r1.extend(r2);
                        r1
                    })
                    .collect()
            })
        })
        .collect()
}

#[derive(Clone)]
struct Pattern {
    indices: Vec<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Pattern {
    fn base() -> Self {
        Self {
            indices: vec![
                (0, 18),
                (1, 0),
                (1, 5),
                (1, 6),
                (1, 11),
                (1, 12),
                (1, 17),
                (1, 18),
                (1, 19),
                (2, 1),
                (2, 4),
                (2, 7),
                (2, 10),
                (2, 13),
                (2, 16),
            ],
            height: 3,
            width: 20,
        }
    }

    fn flipped(&self) -> Self {
        Self {
            indices: self
                .indices
                .iter()
                .map(|(y, x)| (self.height - y, *x))
                .collect(),
            height: self.height,
            width: self.width,
        }
    }

    fn rotated(&self) -> Self {
        Self {
            indices: self
                .indices
                .iter()
                .map(|(y, x)| (*x, self.height - y))
                .collect(),
            height: self.width,
            width: self.height,
        }
    }

    fn test(&self, image: &[Vec<bool>], off_y: usize, off_x: usize) -> bool {
        self.indices
            .iter()
            .all(|(y, x)| image[y + off_y][x + off_x])
    }
    fn mark(&self, monsters: &mut Vec<Vec<bool>>, off_y: usize, off_x: usize) {
        self.indices
            .iter()
            .for_each(|(y, x)| monsters[y + off_y][x + off_x] = true)
    }

    fn detect_monsters(&self, image: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let mut monsters: Vec<Vec<bool>> = (0..image.len())
            .map(|_| (0..image[0].len()).map(|_| false).collect())
            .collect();

        for y in 0..image.len() - self.height {
            for x in 0..image[0].len() - self.width {
                if self.test(image, y, x) {
                    self.mark(&mut monsters, y, x);
                }
            }
        }

        monsters
    }
}

fn patterns() -> Vec<Pattern> {
    let p1 = Pattern::base();
    let p1f = p1.flipped();
    let p2 = p1.rotated();
    let p2f = p1f.rotated();
    let p3 = p2.rotated();
    let p3f = p2f.rotated();
    let p4 = p3.rotated();
    let p4f = p3f.rotated();

    vec![p1, p2, p3, p4, p1f, p2f, p3f, p4f]
}

pub fn run(input: &str) -> usize {
    let tiles = parse_tiles(input);
    let neighbors = find_neighbors(&tiles);
    let image = create_image(&neighbors);

    patterns()
        .iter()
        .map(|p| p.detect_monsters(&image))
        .map(|monsters| {
            (0..image.len())
                .map(|y| {
                    (0..image[0].len())
                        .filter(|x| image[y][*x] && !monsters[y][*x])
                        .count()
                })
                .sum()
        })
        .min()
        .unwrap()
}
