pub mod part1;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 11_576_351, include_str!("input.txt")),
        (part1_example, 14_897_079, include_str!("input.example.txt")),
    );
}
