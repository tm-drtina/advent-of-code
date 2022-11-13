pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 3780860499, include_str!("input.txt")),
        (part1_sanity1, 99, "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 33343, include_str!("input.txt")),
    );
}
