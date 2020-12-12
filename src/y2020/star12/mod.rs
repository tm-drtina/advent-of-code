pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 1645;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 35292;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity() {
        let expected = 286;
        let actual = super::part2::run("F10\nN3\nF7\nR90\nF11");
        assert_eq!(expected, actual);
    }
}
