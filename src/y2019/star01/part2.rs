fn compute_fuel(mass: i32) -> i32 {
    let module_fuel = mass / 3 - 2;
    if module_fuel > 8 {
        module_fuel + compute_fuel(module_fuel)
    } else {
        module_fuel
    }
}

pub fn run(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .map(compute_fuel)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compute_fuel() {
        assert_eq!(2, super::compute_fuel(14));
        assert_eq!(966, super::compute_fuel(1969));
        assert_eq!(50346, super::compute_fuel(100756));
    }
}
