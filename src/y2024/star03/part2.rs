use anyhow::Result;
use regex::Regex;

enum Op {
    Do, Dont, Mul(usize, usize)
}

pub fn run(input: &str) -> Result<usize> {
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do\(\))()()|(don't\(\))()()").unwrap();

    Ok(re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [op, a, b])| {
            match op {
                "mul" => Ok(Op::Mul(a.parse()?, b.parse()?)),
                "do()" => Ok(Op::Do),
                "don't()" => Ok(Op::Dont),
                _ => unreachable!()
            }
        })
        .collect::<Result<Vec<Op>>>()?
        .into_iter()
        .fold((true, 0), |(enabled, sum), op| {
            match op {
                Op::Do => (true, sum),
                Op::Dont => (false, sum),
                Op::Mul(a, b) if enabled => (enabled, sum + a * b),
                Op::Mul(_, _) => (enabled, sum),
            }
        })
        .1
    )
}
