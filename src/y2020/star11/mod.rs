pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 2470;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 37;
        let actual = super::part1::run(include_str!("input.test.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 2259;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 26;
        let actual = super::part2::run(include_str!("input.test.txt"));
        assert_eq!(expected, actual);
    }
}
