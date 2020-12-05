use itertools::Itertools;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

#[derive(Default)]
struct PixelCounts {
    black: i32,
    white: i32,
    transparent: i32,
}

pub fn run(input: &str) -> i32 {
    input
        .chars()
        .chunks(HEIGHT * WIDTH)
        .into_iter()
        .map(|layer| {
            layer
                .into_iter()
                .fold(PixelCounts::default(), |mut acc, ch| {
                    match ch {
                        '0' => acc.black += 1,
                        '1' => acc.white += 1,
                        '2' => acc.transparent += 1,
                        _ => panic!("Unknown image pixel {}", ch),
                    }
                    acc
                })
        })
        .min_by_key(|counts| counts.black)
        .map(|counts| counts.white * counts.transparent)
        .unwrap()
}
