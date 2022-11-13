use std::collections::HashSet;

struct Input<'a, T: Iterator<Item = &'a str>> {
    data: T,
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Input<'a, T> {
    type Item = HashSet<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: HashSet<char> = HashSet::new();
        loop {
            match self.data.next() {
                None => break,
                Some(line) => {
                    if line.is_empty() {
                        break;
                    }
                    for entry in line.chars() {
                        res.insert(entry);
                    }
                }
            }
        }

        if res.is_empty() { None } else { Some(res) }
    }
}

pub fn run(input: &str) -> usize {
    Input {
        data: input.lines(),
    }
    .map(|group| group.len())
    .sum()
}
