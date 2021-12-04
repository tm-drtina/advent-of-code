pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1_sanity() {
        let expected = 4512;
        let actual = super::part1::run(include_str!("input.sanity.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1() {
        let expected = 33348;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_sanity() {
        let expected = 1924;
        let actual = super::part2::run(include_str!("input.sanity.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 8112;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
}