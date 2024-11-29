use std::collections::HashMap;

struct Input<'a, T: Iterator<Item = &'a str>> {
    data: T,
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Input<'a, T> {
    type Item = HashMap<&'a str, &'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: HashMap<&str, &str> = HashMap::new();
        for line in self.data.by_ref() {
            if line.is_empty() {
                break;
            }
            for entry in line.split(' ') {
                let (key, val) = entry.split_once(':').unwrap();
                if res.insert(key, val).is_some() {
                    res.insert("error", "aaa");
                }
            }
        }

        if res.is_empty() { None } else { Some(res) }
    }
}

fn valid_byr(map: &HashMap<&str, &str>) -> bool {
    map.get("byr").is_some_and(|val| {
        val.len() == 4
            && val
                .parse()
                .is_ok_and(|ival| (1920..=2002).contains(&ival))
    })
}
fn valid_iyr(map: &HashMap<&str, &str>) -> bool {
    map.get("iyr").is_some_and(|val| {
        val.len() == 4
            && val
                .parse()
                .is_ok_and(|ival| (2010..=2020).contains(&ival))
    })
}
fn valid_eyr(map: &HashMap<&str, &str>) -> bool {
    map.get("eyr").is_some_and(|val| {
        val.len() == 4
            && val
                .parse()
                .is_ok_and(|ival| (2020..=2030).contains(&ival))
    })
}
fn valid_hgt(map: &HashMap<&str, &str>) -> bool {
    map.get("hgt").is_some_and(|val| {
        val.len() > 3
            && match &val[val.len() - 2..] {
                "in" => val[0..val.len() - 2]
                    .parse()
                    .is_ok_and(|num| (59..=76).contains(&num)),
                "cm" => val[0..val.len() - 2]
                    .parse()
                    .is_ok_and(|num| (150..=193).contains(&num)),
                _ => false,
            }
    })
}
fn valid_hcl(map: &HashMap<&str, &str>) -> bool {
    map.get("hcl").is_some_and(|val| {
        val.len() == 7 && val.starts_with('#') && val[1..].chars().all(|ch| ch.is_ascii_hexdigit())
    })
}
fn valid_ecl(map: &HashMap<&str, &str>) -> bool {
    map.get("ecl").is_some_and(|val| {
        matches!(*val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    })
}
fn valid_pid(map: &HashMap<&str, &str>) -> bool {
    map.get("pid").is_some_and(|val| {
        val.len() == 9 && val.chars().all(char::is_numeric)
    })
}

pub fn run(input: &str) -> usize {
    Input {
        data: input.lines(),
    }
    .filter(valid_byr)
    .filter(valid_iyr)
    .filter(valid_eyr)
    .filter(valid_hgt)
    .filter(valid_hcl)
    .filter(valid_ecl)
    .filter(valid_pid)
    .count()
}
