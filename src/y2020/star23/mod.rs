pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = "43896725";
        let actual = super::part1::run("589174263");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = "67384529";
        let actual = super::part1::run("389125467");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 2911418906;
        let actual = super::part2::run("589174263");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 149245887792;
        let actual = super::part2::run("389125467");
        assert_eq!(expected, actual);
    }
}
