pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 6504;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 11;
        let actual = super::part1::run("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 3351;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 6;
        let actual = super::part2::run("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
        assert_eq!(expected, actual);
    }
}
