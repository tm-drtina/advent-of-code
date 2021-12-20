use super::part1::Image;

pub fn run(input: &str) -> usize {
    let (alg, input) = input.split_once("\n\n").unwrap();
    let alg = alg.chars().map(|ch| ch == '#').collect::<Vec<_>>();
    let image = input.parse::<Image>().unwrap();

    (0..50)
        .fold(image, |image, _| image.enhance(&alg))
        .count_white()
}
