pub mod part1;
pub mod part2;
pub mod part2_overengineered;

#[macro_use]
#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 343605, include_str!("input.txt")),
        (part1_sanity, 37, include_str!("input.sanity.txt"))
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 96744904, include_str!("input.txt")),
        (part2_sanity, 168, include_str!("input.sanity.txt")),
        (part2_sanity2, 4611686018427387904, "0,4294967295"),
        (part2_sanity3, 2, "0,1,2"),
        (part2_sanity4, 38, "0,1,2,10"),
    );

    aoc_test_suite!(
        super::part2_overengineered::run,
        (part2_overeng_main, 96744904, include_str!("input.txt")),
        (part2_overeng_sanity, 168, include_str!("input.sanity.txt")),
        (part2_overeng_sanity2, 4611686018427387904, "0,4294967295"),
        (part2_overeng_sanity3, 2, "0,1,2"),
        (part2_overeng_sanity4, 38, "0,1,2,10"),
    );
}
