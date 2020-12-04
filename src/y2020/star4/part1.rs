use std::collections::HashMap;

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

pub fn run(input: &str) -> usize {
    Input {
        data: input.lines(),
    }
    .filter(|passport| passport.contains_key("byr"))
    .filter(|passport| passport.contains_key("iyr"))
    .filter(|passport| passport.contains_key("eyr"))
    .filter(|passport| passport.contains_key("hgt"))
    .filter(|passport| passport.contains_key("hcl"))
    .filter(|passport| passport.contains_key("ecl"))
    .filter(|passport| passport.contains_key("pid"))
    .count()
}
