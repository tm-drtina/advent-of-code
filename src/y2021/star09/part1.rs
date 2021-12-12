pub fn run(input: &str) -> u32 {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().map(|ch| ch as u32 - '0' as u32).collect())
        .collect();
    let mut sum = 0;
    for (rowi, row) in map.iter().enumerate() {
        for (coli, col) in row.iter().enumerate() {
            let top = rowi == 0 || map[rowi - 1][coli] > *col;
            let bot = rowi == map.len() - 1 || map[rowi + 1][coli] > *col;
            let left = coli == 0 || row[coli - 1] > *col;
            let right = coli == row.len() - 1 || row[coli + 1] > *col;
            if top && bot && left && right {
                sum += *col + 1;
            }
        }
    }
    sum
}
