pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 524;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 2;
        let actual = super::part1::run("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 485;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 1;
        let actual = super::part2::run("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(expected, actual);
    }
}
