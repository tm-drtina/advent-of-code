pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 26869;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 71;
        let actual = super::part1::run(include_str!("input.test.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 855275529001;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity1() {
        let expected = 12;
        let actual = super::part2::run(include_str!("input.test2_1.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity2() {
        let expected = 11;
        let actual = super::part2::run(include_str!("input.test2_2.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity3() {
        let expected = 13;
        let actual = super::part2::run(include_str!("input.test2_3.txt"));
        assert_eq!(expected, actual);
    }
}
