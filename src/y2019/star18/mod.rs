pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1_sanity1() {
        let expected = 8;
        let actual = super::part1::run(include_str!("input.sanity1.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity2() {
        let expected = 86;
        let actual = super::part1::run(include_str!("input.sanity2.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity3() {
        let expected = 132;
        let actual = super::part1::run(include_str!("input.sanity3.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity4() {
        let expected = 136;
        let actual = super::part1::run(include_str!("input.sanity4.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity5() {
        let expected = 81;
        let actual = super::part1::run(include_str!("input.sanity5.txt"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_main() {
        let expected = 4246;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_sanity6() {
        let expected = 8;
        let actual = super::part2::run(include_str!("input.sanity6.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity7() {
        let expected = 24;
        let actual = super::part2::run(include_str!("input.sanity7.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity8() {
        let expected = 32;
        let actual = super::part2::run(include_str!("input.sanity8.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity9() {
        let expected = 72;
        let actual = super::part2::run(include_str!("input.sanity9.txt"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_main() {
        let expected = 1940;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
}