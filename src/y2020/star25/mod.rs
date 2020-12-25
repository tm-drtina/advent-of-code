pub mod part1;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 11576351;
        let actual = super::part1::run("2069194\n16426071");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity() {
        let expected = 14897079;
        let actual = super::part1::run("5764801\n17807724");
        assert_eq!(expected, actual);
    }
}
