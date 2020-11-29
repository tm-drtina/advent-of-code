pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 966;
        let actual = super::part1::run(264793, 803935);
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 628;
        let actual = super::part2::run(264793, 803935);
        assert_eq!(expected, actual);
    }
}
