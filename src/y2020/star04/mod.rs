pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 213;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 2;
        let actual = super::part1::run(include_str!("input-test.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 147;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity1() {
        let expected = 0;
        let actual = super::part2::run(include_str!("input-test2.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity3() {
        let expected = 4;
        let actual = super::part2::run(include_str!("input-test3.txt"));
        assert_eq!(expected, actual);
    }
}
