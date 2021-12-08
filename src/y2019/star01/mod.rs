pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 3425624;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 5135558;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
}
