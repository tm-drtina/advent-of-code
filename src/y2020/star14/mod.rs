pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn sanity() {
        assert_eq!(u64::MAX - 1, u64::MAX << 1);
        assert_eq!(u64::MAX, (u64::MAX << 1) | 1);
        assert_eq!(0_u64, u64::MIN << 1);
        assert_eq!(1_u64, (u64::MIN << 1) | 1);
        assert_eq!(73_u64, (11_u64 | 64_u64) & ((u64::MAX << 2) | 1));
    }
    #[test]
    fn part1() {
        let expected = 13476250121721;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 165;
        let actual = super::part1::run(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0",
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 4463708436768;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 208;
        let actual = super::part2::run("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1");
        assert_eq!(expected, actual);
    }
}
