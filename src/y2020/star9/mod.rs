pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 3199139634;
        let actual = super::part1::run(include_str!("input.txt"), 25);
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 127;
        let actual = super::part1::run(include_str!("input.test.txt"), 5);
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 438559930;
        let actual = super::part2::run(include_str!("input.txt"), 25);
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 62;
        let actual = super::part2::run(include_str!("input.test.txt"), 5);
        assert_eq!(expected, actual);
    }
}
