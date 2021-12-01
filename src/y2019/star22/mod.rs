pub mod part1;
pub mod part2;
pub mod part2_v3;

#[cfg(test)]
mod tests {
    fn sanity<F: Fn(usize) -> usize>(expected_order: Vec<usize>, f: F) {
        let deck_size = expected_order.len();
        let mut actual_order: Vec<usize> = (0..deck_size).map(|_| usize::MAX).collect();

        for position in 0..deck_size {
            actual_order[f(position)] = position;
        }
        assert_eq!(expected_order, actual_order);
    }

    fn part1_sanity(input: &str, expected_order: Vec<usize>) {
        let deck_size = expected_order.len();
        sanity(expected_order, |position| super::part1::run(input, position, deck_size));
    }

    #[test]
    fn part1_sanity_new_stack() {
        let expected_order = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        part1_sanity("deal into new stack", expected_order);
    }
    #[test]
    fn part1_sanity_cut3() {
        let expected_order = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        part1_sanity("cut 3", expected_order);
    }
    #[test]
    fn part1_sanity_cut_neg4() {
        let expected_order = vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        part1_sanity("cut -4", expected_order);
    }
    #[test]
    fn part1_sanity_inc3() {
        let expected_order = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
        part1_sanity("deal with increment 3", expected_order);
    }

    #[test]
    fn part1_sanity1() {
        let expected_order = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        part1_sanity(include_str!("input.sanity1.txt"), expected_order);
    }
    #[test]
    fn part1_sanity2() {
        let expected_order = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        part1_sanity(include_str!("input.sanity2.txt"), expected_order);
    }
    #[test]
    fn part1_sanity3() {
        let expected_order = vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
        part1_sanity(include_str!("input.sanity3.txt"), expected_order);
    }
    #[test]
    fn part1_sanity4() {
        let expected_order = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
        part1_sanity(include_str!("input.sanity4.txt"), expected_order);
    }

    #[test]
    fn part1_main() {
        let expected = 7614;
        let actual = super::part1::run(include_str!("input.txt"), 2019, 10007);
        assert_eq!(expected, actual);
    }


    fn part2_sanity(input: &str, expected_order: Vec<usize>, repetitions: usize) {
        let deck_size = expected_order.len();
        sanity(expected_order, |position| super::part2_v3::run(input, position, deck_size, repetitions));
    }

    #[test]
    fn part2_sanity_new_stack1() {
        let expected_order = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        part2_sanity("deal into new stack", expected_order, 1);
    }
    #[test]
    fn part2_sanity_new_stack2() {
        let expected_order = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        part2_sanity("deal into new stack", expected_order, 2);
    }
    #[test]
    fn part2_sanity_new_stack3() {
        let expected_order = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        part2_sanity("deal into new stack", expected_order, 3);
    }
    #[test]
    fn part2_sanity_new_stack4() {
        let expected_order = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        part2_sanity("deal into new stack", expected_order, 4);
    }
    #[test]
    fn part2_sanity_new_stack5() {
        let expected_order = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        part2_sanity("deal into new stack", expected_order, 5);
    }
    #[test]
    fn part2_sanity_cut3() {
        let expected_order = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        part2_sanity("cut 3", expected_order, 1);
    }
    #[test]
    fn part2_sanity_cut_neg4() {
        let expected_order = vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        part2_sanity("cut -4", expected_order, 1);
    }
    #[test]
    fn part2_sanity_inc3() {
        let expected_order = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
        part2_sanity("deal with increment 3", expected_order, 1);
    }

    #[test]
    fn part2_sanity1() {
        let expected_order = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        part2_sanity(include_str!("input.sanity1.txt"), expected_order, 1);
    }
    #[test]
    fn part2_sanity2() {
        let expected_order = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        part2_sanity(include_str!("input.sanity2.txt"), expected_order, 1);
    }
    #[test]
    fn part2_sanity3() {
        let expected_order = vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
        part2_sanity(include_str!("input.sanity3.txt"), expected_order, 1);
    }
    #[test]
    fn part2_sanity4() {
        let expected_order = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
        part2_sanity(include_str!("input.sanity4.txt"), expected_order, 1);
    }

    #[test]
    fn part2_main() {
        let expected = 0;
        let actual = super::part2_v3::run(include_str!("input.txt"), 2020, 119315717514047, 101741582076661);
        assert!(actual > 35377754932371usize);
        assert!(actual < 102581697963382usize);
        assert_eq!(expected, actual);
    }
}
