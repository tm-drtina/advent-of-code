pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 391;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 10;
        let actual = super::part1::run(include_str!("input.test.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 3876;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 2208;
        let actual = super::part2::run(include_str!("input.test.txt"));
        assert_eq!(expected, actual);
    }
}
