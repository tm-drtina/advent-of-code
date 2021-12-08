use std::collections::HashMap;

pub struct Tile {
    pub data: Vec<Vec<bool>>,
}

impl Tile {
    pub fn up_line(&self) -> Vec<bool> {
        self.data.first().unwrap().clone()
    }
    pub fn down_line(&self) -> Vec<bool> {
        self.data.last().unwrap().clone()
    }
    pub fn left_line(&self) -> Vec<bool> {
        self.data.iter().map(|x| *x.first().unwrap()).collect()
    }
    pub fn right_line(&self) -> Vec<bool> {
        self.data.iter().map(|x| *x.last().unwrap()).collect()
    }
    pub fn is_neighbor(&self, line: &[bool]) -> bool {
        let mut line_reversed = line.to_owned();
        line_reversed.reverse();

        let other = self.up_line();
        if line == other || line_reversed == other {
            return true;
        }

        let other = self.down_line();
        if line == other || line_reversed == other {
            return true;
        }

        let other = self.left_line();
        if line == other || line_reversed == other {
            return true;
        }

        let other = self.right_line();
        if line == other || line_reversed == other {
            return true;
        }

        false
    }
}

pub fn parse_tiles(input: &str) -> HashMap<i32, Tile> {
    let mut res = HashMap::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let id = line[5..line.len() - 1].parse().unwrap();
        let mut data: Vec<Vec<bool>> = Vec::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            data.push(line.chars().map(|ch| ch == '#').collect());
        }
        res.insert(id, Tile { data });
    }
    res
}

#[derive(Clone)]
pub struct Neighbors {
    pub id: i32,
    pub data: Vec<Vec<bool>>,
    pub up: Option<i32>,
    pub down: Option<i32>,
    pub left: Option<i32>,
    pub right: Option<i32>,
}

impl Neighbors {
    fn num_neighbors(&self) -> u8 {
        let mut res = 0;
        if self.up.is_some() {
            res += 1;
        }
        if self.down.is_some() {
            res += 1;
        }
        if self.left.is_some() {
            res += 1;
        }
        if self.right.is_some() {
            res += 1;
        }
        res
    }
    pub fn is_corner(&self) -> bool {
        self.num_neighbors() == 2
    }
}

pub fn find_neighbors(tiles: &HashMap<i32, Tile>) -> HashMap<i32, Neighbors> {
    let mut res = HashMap::new();
    for (id, tile) in tiles {
        let up = tile.up_line();
        let down = tile.down_line();
        let left = tile.left_line();
        let right = tile.right_line();

        let neighbors = Neighbors {
            id: *id,
            data: tile.data.clone(),
            up: tiles
                .iter()
                .find(|(k, x)| *k != id && x.is_neighbor(&up))
                .map(|(key, _)| *key),
            down: tiles
                .iter()
                .find(|(k, x)| *k != id && x.is_neighbor(&down))
                .map(|(key, _)| *key),
            left: tiles
                .iter()
                .find(|(k, x)| *k != id && x.is_neighbor(&left))
                .map(|(key, _)| *key),
            right: tiles
                .iter()
                .find(|(k, x)| *k != id && x.is_neighbor(&right))
                .map(|(key, _)| *key),
        };
        res.insert(*id, neighbors);
    }
    res
}

pub fn run(input: &str) -> i64 {
    let tiles = parse_tiles(input);
    let neighbors = find_neighbors(&tiles);

    neighbors
        .iter()
        .filter(|(_, n)| n.is_corner())
        .map(|(id, _)| *id as i64)
        .product()
}
