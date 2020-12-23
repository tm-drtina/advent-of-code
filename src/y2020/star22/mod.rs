pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 35013;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 306;
        let actual = super::part1::run(include_str!("input.test.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 32806;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 291;
        let actual = super::part2::run(include_str!("input.test.txt"));
        assert_eq!(expected, actual);
    }
}
