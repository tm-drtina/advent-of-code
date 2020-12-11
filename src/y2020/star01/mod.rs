pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 921504;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 514579;
        let actual = super::part1::run("1721\n979\n366\n299\n675\n1456");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 195700142;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 241861950;
        let actual = super::part2::run("1721\n979\n366\n299\n675\n1456");
        assert_eq!(expected, actual);
    }
}
