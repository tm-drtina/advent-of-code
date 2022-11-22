pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 913, include_str!("input.txt")),
        (part1_sanity1, 16, "8A004A801A8002F478"),
        (part1_sanity2, 12, "620080001611562C8802118E34"),
        (part1_sanity3, 23, "C0015000016115A2E0802F182340"),
        (part1_sanity4, 31, "A0016C880162017C3686B18A3D4780"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1510977819698, include_str!("input.txt")),
        (part2_sanity1, 3, "C200B40A82"),
        (part2_sanity2, 54, "04005AC33890"),
        (part2_sanity3, 7, "880086C3E88112"),
        (part2_sanity4, 9, "CE00C43D881120"),
        (part2_sanity5, 1, "D8005AC2A8F0"),
        (part2_sanity6, 0, "F600BC2D8F"),
        (part2_sanity7, 0, "9C005AC2F8F0"),
        (part2_sanity8, 1, "9C0141080250320F1802104A08"),
    );
}
