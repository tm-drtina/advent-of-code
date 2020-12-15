pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 929;
        let actual = super::part1::run("16,1,0,18,12,14,19");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        assert_eq!(436, super::part1::run("0,3,6"));
        assert_eq!(1, super::part1::run("1,3,2"));
        assert_eq!(10, super::part1::run("2,1,3"));
        assert_eq!(27, super::part1::run("1,2,3"));
        assert_eq!(78, super::part1::run("2,3,1"));
        assert_eq!(438, super::part1::run("3,2,1"));
        assert_eq!(1836, super::part1::run("3,1,2"));
    }

    #[test]
    fn part2() {
        let expected = 16671510;
        let actual = super::part2::run("16,1,0,18,12,14,19");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity1() {
        assert_eq!(175594, super::part2::run("0,3,6"));
    }
    #[test]
    fn part2_sanity2() {
        assert_eq!(2578, super::part2::run("1,3,2"));
    }
    #[test]
    fn part2_sanity3() {
        assert_eq!(3544142, super::part2::run("2,1,3"));
    }
    #[test]
    fn part2_sanity4() {
        assert_eq!(261214, super::part2::run("1,2,3"));
    }
    #[test]
    fn part2_sanity5() {
        assert_eq!(6895259, super::part2::run("2,3,1"));
    }
    #[test]
    fn part2_sanity6() {
        assert_eq!(18, super::part2::run("3,2,1"));
    }
    #[test]
    fn part2_sanity7() {
        assert_eq!(362, super::part2::run("3,1,2"));
    }
}
