use itertools::Itertools;

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/

const NUMBERS: [usize; 10] = [
    0b111_0111, //0
    0b001_0010, //1
    0b101_1101, //2
    0b101_1011, //3
    0b011_1010, //4
    0b110_1011, //5
    0b110_1111, //6
    0b101_0010, //7
    0b111_1111, //8
    0b111_1011, //9
];

fn convert(num: &[usize], mapper: &[usize]) -> Option<usize> {
    let mapped = num
        .iter()
        .copied()
        .map(|n| mapper[n])
        .fold(0usize, |acc, val| acc + (1 << val));
    (0..10).find(|v| NUMBERS[*v] == mapped)
}

fn compute(train: &[Vec<usize>], test: &[Vec<usize>]) -> usize {
    let mapper = (0..7)
        .permutations(7)
        .find(|mapper| {
            train.iter().all(|num| convert(num, mapper).is_some())
                && test.iter().all(|num| convert(num, mapper).is_some())
        })
        .unwrap();
    test.iter()
        .fold(0, |acc, val| acc * 10 + convert(val, &mapper).unwrap())
}

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (train, test) = line.split_once(" | ").unwrap();
            let train: Vec<Vec<usize>> = train
                .split(' ')
                .map(|s| s.chars().map(|ch| ch as usize - 'a' as usize).collect())
                .collect();
            let test: Vec<Vec<usize>> = test
                .split(' ')
                .map(|s| s.chars().map(|ch| ch as usize - 'a' as usize).collect())
                .collect();
            compute(&train, &test)
        })
        .sum()
}
