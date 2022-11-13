use std::collections::HashMap;

fn compute_orbits(indirect: i32, name: &str, map: &HashMap<&str, Vec<&str>>) -> i32 {
    map.get(name)
        .unwrap_or(&Vec::new())
        .iter()
        .fold(0, |acc, subname| {
            acc + indirect + compute_orbits(indirect + 1, subname, map)
        })
}

pub fn run(input: &str) -> i32 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once(')').unwrap();
        map.entry(a).or_default().push(b);
    });
    compute_orbits(1, "COM", &map)
}
