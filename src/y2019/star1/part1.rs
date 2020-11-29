use std::str::FromStr;

fn compute_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn run(input: &str) -> i32 {
    input.lines()
        .map(|x| i32::from_str(x).unwrap())
        .map(|mass| compute_fuel(mass))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compute_fuel() {
        assert_eq!(2, super::compute_fuel(12));
        assert_eq!(2, super::compute_fuel(14));
        assert_eq!(654, super::compute_fuel(1969));
        assert_eq!(33583, super::compute_fuel(100756));
    }
}
