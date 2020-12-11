use std::collections::HashMap;
use std::str::FromStr;

struct Input<'a, T: Iterator<Item = &'a str>> {
    data: T,
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Input<'a, T> {
    type Item = HashMap<&'a str, &'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: HashMap<&str, &str> = HashMap::new();
        loop {
            match self.data.next() {
                None => break,
                Some(line) => {
                    if line.is_empty() {
                        break;
                    }
                    for entry in line.split(' ') {
                        let (key, val) = entry.split_once(':').unwrap();
                        let foo = res.insert(key, val);
                        if foo.is_some() {
                            res.insert("error", "aaa");
                        }
                    }
                }
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

fn valid_byr(map: &HashMap<&str, &str>) -> bool {
    map.get("byr")
        .map(|val| {
            val.len() == 4
                && i32::from_str(val)
                    .map(|ival| ival >= 1920 && ival <= 2002)
                    .unwrap_or(false)
        })
        .unwrap_or(false)
}
fn valid_iyr(map: &HashMap<&str, &str>) -> bool {
    map.get("iyr")
        .map(|val| {
            val.len() == 4
                && i32::from_str(val)
                    .map(|ival| ival >= 2010 && ival <= 2020)
                    .unwrap_or(false)
        })
        .unwrap_or(false)
}
fn valid_eyr(map: &HashMap<&str, &str>) -> bool {
    map.get("eyr")
        .map(|val| {
            val.len() == 4
                && i32::from_str(val)
                    .map(|ival| ival >= 2020 && ival <= 2030)
                    .unwrap_or(false)
        })
        .unwrap_or(false)
}
fn valid_hgt(map: &HashMap<&str, &str>) -> bool {
    map.get("hgt")
        .map(|val| {
            val.len() > 3
                && match &val[val.len() - 2..] {
                    "in" => i32::from_str(&val[0..val.len() - 2])
                        .map_or(false, |num| num >= 59 && num <= 76),
                    "cm" => i32::from_str(&val[0..val.len() - 2])
                        .map_or(false, |num| num >= 150 && num <= 193),
                    _ => false,
                }
        })
        .unwrap_or(false)
}
fn valid_hcl(map: &HashMap<&str, &str>) -> bool {
    map.get("hcl")
        .map(|val| {
            val.len() == 7
                && val.chars().next().unwrap() == '#'
                && val[1..].chars().all(|ch| ch.is_ascii_hexdigit())
        })
        .unwrap_or(false)
}
fn valid_ecl(map: &HashMap<&str, &str>) -> bool {
    map.get("ecl")
        .map(|val| matches!(*val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"))
        .unwrap_or(false)
}
fn valid_pid(map: &HashMap<&str, &str>) -> bool {
    map.get("pid")
        .map(|val| val.len() == 9 && val.chars().all(|ch| ch.is_numeric()))
        .unwrap_or(false)
}

pub fn run(input: &str) -> usize {
    Input {
        data: input.lines(),
    }
    .filter(|passports| valid_byr(passports))
    .filter(|passports| valid_iyr(passports))
    .filter(|passports| valid_eyr(passports))
    .filter(|passports| valid_hgt(passports))
    .filter(|passports| valid_hcl(passports))
    .filter(|passports| valid_ecl(passports))
    .filter(|passports| valid_pid(passports))
    .count()
}
