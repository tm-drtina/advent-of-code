pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 822;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        assert_eq!(567, super::part1::run("BFFFBBFRRR"));
        assert_eq!(119, super::part1::run("FFFBBBFRRR"));
        assert_eq!(820, super::part1::run("BBFFBBFRLL"));
    }
    #[test]
    fn part2() {
        let expected = 705;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
}
