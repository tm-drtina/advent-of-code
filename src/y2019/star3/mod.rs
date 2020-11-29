pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 768;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity1() {
        let expected = 6;
        let actual = super::part1::run("R8,U5,L5,D3\nU7,R6,D4,L4");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity2() {
        let expected = 159;
        let actual = super::part1::run(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn part1_sanity3() {
        let expected = 135;
        let actual = super::part1::run(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = 8684;
        let actual = super::part2::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity1() {
        let expected = 30;
        let actual = super::part2::run("R8,U5,L5,D3\nU7,R6,D4,L4");
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity2() {
        let expected = 610;
        let actual = super::part2::run(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2_sanity3() {
        let expected = 410;
        let actual = super::part2::run(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        assert_eq!(expected, actual);
    }
}
