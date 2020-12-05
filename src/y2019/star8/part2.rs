use itertools::Itertools;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

pub fn run(input: &str) -> String {
    input
        .chars()
        .chunks(HEIGHT * WIDTH)
        .into_iter()
        .fold(vec!['2'; HEIGHT * WIDTH], |acc, layer| {
            acc.iter()
                .zip(layer)
                .map(|(a, b)| if *a == '2' { b } else { *a })
                .collect()
        })
        .iter()
        .map(|ch| match ch {
            '0' => " ",
            '1' => "#",
            _ => panic!("Unknown image pixel {}", ch),
        })
        .chunks(WIDTH)
        .into_iter()
        .map(|mut line| line.join(""))
        .join("\n")
}
