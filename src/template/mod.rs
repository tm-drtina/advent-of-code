pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = "Part1 result: Data for the first part";
        let actual = super::part1::run(include_str!("input.part1.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = "Part2 result: Data for the second part";
        let actual = super::part2::run(include_str!("input.part2.txt"));
        assert_eq!(expected, actual);
    }
}