pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 2020, include_str!("input.txt")),
        (part1_sanity, 5, include_str!("input.sanity.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (
            part2_main,
            "bcdgf,xhrdsl,vndrb,dhbxtb,lbnmsr,scxxn,bvcrrfbr,xcgtv",
            include_str!("input.txt")
        ),
        (
            part2_sanity,
            "mxmxvkd,sqjhc,fvjkl",
            include_str!("input.sanity.txt")
        ),
    );
}
