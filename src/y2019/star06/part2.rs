use std::collections::HashMap;

fn compute_jumps(name: &str, map: &HashMap<&str, Vec<&str>>) -> (Option<i32>, Option<i32>) {
    let mut depth_you: Option<i32> = None;
    let mut depth_san: Option<i32> = None;

    for x in map.get(name).unwrap_or(&Vec::new()) {
        let (res_you, res_san) = compute_jumps(x, map);
        if res_you.is_some() && res_san.is_some() {
            return (res_you, res_san);
        }
        if let Some(you) = res_you {
            depth_you = Some(you + 1)
        }
        if let Some(san) = res_san {
            depth_san = Some(san + 1)
        }
    }

    if name == "YOU" {
        depth_you = Some(-1)
    }
    if name == "SAN" {
        depth_san = Some(-1)
    }

    (depth_you, depth_san)
}

pub fn run(input: &str) -> i32 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once(')').unwrap();
        map.entry(a).or_default().push(b);
    });
    let (res_you, res_san) = compute_jumps("COM", &map);
    res_you.unwrap() + res_san.unwrap()
}
