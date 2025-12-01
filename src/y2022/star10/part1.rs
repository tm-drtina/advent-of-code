use anyhow::{Result, bail};

struct State {
    x: i32,
    counter: i32,
    result: i32,
    next_result_counter: i32,
}

impl State {
    fn new() -> Self {
        Self {
            x: 1,
            counter: 0,
            result: 0,
            next_result_counter: 20,
        }
    }

    fn compute_result(&mut self) {
        if self.counter >= self.next_result_counter {
            self.result += self.x * (self.counter - (self.counter % 20));
            self.next_result_counter += 40;
        }
    }

    fn noop(&mut self) {
        self.counter += 1;
        self.compute_result();
    }

    fn addx(&mut self, amount: i32) {
        self.counter += 2;
        self.compute_result();
        self.x += amount;
    }

    fn step(&mut self, line: &str) -> Result<()> {
        match line {
            "noop" => self.noop(),
            _ if line.starts_with("addx ") => {
                self.addx(line.strip_prefix("addx ").unwrap().parse()?);
            }
            _ => bail!("Unknown command {}", line),
        }
        Ok(())
    }
}

pub fn run(input: &str) -> Result<i32> {
    let mut state = State::new();
    for line in input.lines() {
        state.step(line)?;
    }
    Ok(state.result)
}
