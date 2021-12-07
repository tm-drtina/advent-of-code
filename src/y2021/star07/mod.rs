pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1_sanity() {
        let expected = 37;
        let actual = super::part1::run(include_str!("input.sanity.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_main() {
        let expected = 343605;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_sanity() {
        let expected = 168;
        let actual = super::part2::run(include_str!("input.sanity.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_main() {
        let expected = 96744904;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
}