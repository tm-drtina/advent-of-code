pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 296;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 295;
        let actual = super::part1::run("939\n7,13,x,x,59,x,31,19");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 535296695251210;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 1068781;
        let actual = super::part2::run("939\n7,13,x,x,59,x,31,19");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity2() {
        let expected = 3417;
        let actual = super::part2::run("939\n17,x,13,19");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity3() {
        let expected = 754018;
        let actual = super::part2::run("939\n67,7,59,61");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity4() {
        let expected = 779210;
        let actual = super::part2::run("939\n67,x,7,59,61");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity5() {
        let expected = 1261476;
        let actual = super::part2::run("939\n67,7,x,59,61");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity6() {
        let expected = 1202161486;
        let actual = super::part2::run("939\n1789,37,47,1889");
        assert_eq!(expected, actual);
    }
}
