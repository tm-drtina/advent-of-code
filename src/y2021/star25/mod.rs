pub mod part1;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 486, include_str!("input.txt")),
        (part1_sanity, 58, include_str!("input.sanity.txt")),
    );
}
