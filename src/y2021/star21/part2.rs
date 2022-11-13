use std::collections::HashMap;

use super::part1::Game;

fn dfs(game: Game, cache: &mut HashMap<Game, (usize, usize)>) -> (usize, usize) {
    if game.score2 >= 21 {
        return if game.p1_turn { (0, 1) } else { (1, 0) };
    }
    if let Some(res) = cache.get(&game).copied() {
        return res;
    }
    let mut res = (0, 0);
    for d1 in 1..4 {
        for d2 in 1..4 {
            for d3 in 1..4 {
                let mut g = game.clone();
                g.step(d1 + d2 + d3);
                let (r1, r2) = dfs(g, cache);
                res.0 += r1;
                res.1 += r2;
            }
        }
    }
    cache.insert(game, res);
    res
}

pub fn run(input: &str) -> usize {
    let game: Game = input.parse().unwrap();
    let mut cache = HashMap::new();

    let (s1, s2) = dfs(game, &mut cache);
    s1.max(s2)
}
