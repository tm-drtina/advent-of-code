use anyhow::{Result, bail};

struct State {
    x: i32,
    counter: i32,
    result: String,
}

impl State {
    fn new() -> Self {
        Self {
            x: 1,
            counter: 0,
            result: String::new(),
        }
    }

    fn write_res(&mut self) {
        if self.x.abs_diff(self.counter % 40) < 2 {
            self.result.push('#');
        } else {
            self.result.push('.');
        }
        self.counter += 1;
        if self.counter % 40 == 0 {
            self.result.push('\n');
        }
    }

    fn noop(&mut self) {
        self.write_res();
    }

    fn addx(&mut self, amount: i32) {
        self.write_res();
        self.write_res();
        self.x += amount;
    }

    fn step(&mut self, line: &str) -> Result<()> {
        match line {
            "noop" => self.noop(),
            _ if line.starts_with("addx ") => {
                self.addx(line.strip_prefix("addx ").unwrap().parse()?);
            }
            _ => bail!("Unknown command {line}"),
        }
        Ok(())
    }
}

pub fn run(input: &str) -> Result<String> {
    let mut state = State::new();
    for line in input.lines() {
        state.step(line)?;
    }
    Ok(state.result)
}
