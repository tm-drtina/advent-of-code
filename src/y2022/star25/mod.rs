pub mod part1;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (
            part1_main,
            "2-2--02=1---1200=0-1",
            include_str!("input.txt")
        ),
        (part1_sanity1, "2=-1=0", include_str!("input.sanity1.txt")),
    );
}
