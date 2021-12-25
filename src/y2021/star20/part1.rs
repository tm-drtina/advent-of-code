#[derive(Debug)]
pub(super) struct Image {
    data: Vec<Vec<bool>>,
    around_fill: bool,
}

impl std::str::FromStr for Image {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let around_fill = false;
        let mut data = vec![
            Vec::with_capacity(0), // Placeholder for first empty line
            Vec::with_capacity(0), // Placeholder for first empty line
        ];
        for line in s.lines() {
            let mut row = Vec::with_capacity(line.len() + 4);
            row.push(around_fill);
            row.push(around_fill);
            for ch in line.chars() {
                row.push(match ch {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!("Invalid char"),
                });
            }
            row.push(around_fill);
            row.push(around_fill);
            data.push(row);
        }
        let width = data[2].len();
        data[0] = vec![around_fill; width];
        data[1] = vec![around_fill; width];
        data.push(vec![around_fill; width]);
        data.push(vec![around_fill; width]);

        Ok(Self { data, around_fill })
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.data.len();
        for row in self.data.iter().skip(2).take(height - 4) {
            let width = row.len();
            for value in row.iter().skip(2).take(width - 4) {
                if *value {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Image {
    pub(super) fn enhance(self, alg: &[bool]) -> Self {
        let mut data = Vec::new();
        let width = self.data.first().unwrap().len();
        let new_width = width + 2;
        let around_fill = if self.around_fill {
            alg[0b1_1111_1111]
        } else {
            alg[0]
        };

        data.push(vec![around_fill; new_width]);
        data.push(vec![around_fill; new_width]);

        for y in 1..(self.data.len() - 1) {
            let mut row = Vec::with_capacity(new_width);
            row.push(around_fill);
            row.push(around_fill);
            for x in 1..(width - 1) {
                let mut index = 0;
                for y1 in (y - 1)..(y + 2) {
                    for x1 in (x - 1)..(x + 2) {
                        index <<= 1;
                        if self.data[y1][x1] {
                            index += 1;
                        }
                    }
                }
                row.push(alg[index]);
            }
            row.push(around_fill);
            row.push(around_fill);
            data.push(row);
        }

        data.push(vec![around_fill; new_width]);
        data.push(vec![around_fill; new_width]);

        Self { data, around_fill }
    }

    pub(super) fn count_white(&self) -> usize {
        self.data.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .copied()
                .fold(0, |acc, value| if value { acc + 1 } else { acc })
        })
    }
}

pub fn run(input: &str) -> usize {
    let (alg, input) = input.split_once("\n\n").unwrap();
    let alg = alg.chars().map(|ch| ch == '#').collect::<Vec<_>>();
    // eprintln!("{}", input.parse::<Image>().unwrap());
    // eprintln!("{}", input.parse::<Image>().unwrap().enhance(&alg));
    // eprintln!("{}", input.parse::<Image>().unwrap().enhance(&alg).enhance(&alg));
    input
        .parse::<Image>()
        .unwrap()
        .enhance(&alg)
        .enhance(&alg)
        .count_white()
}
