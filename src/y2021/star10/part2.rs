fn parse(s: &str) -> Option<usize> {
    let mut stack = Vec::<char>::new();
    for char in s.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' if stack.last() == Some(&'(') => {
                stack.pop().unwrap();
            }
            ']' if stack.last() == Some(&'[') => {
                stack.pop().unwrap();
            }
            '}' if stack.last() == Some(&'{') => {
                stack.pop().unwrap();
            }
            '>' if stack.last() == Some(&'<') => {
                stack.pop().unwrap();
            }
            _ => return None, //invalid
        }
    }
    Some(stack.iter().rev().fold(0, |acc, ch| {
        let score = match *ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        };
        acc * 5 + score
    })) // (in)complete, but valid
}

pub fn run(input: &str) -> usize {
    let mut res: Vec<_> = input.lines().filter_map(parse).collect();
    res.sort();
    res[res.len() / 2]
}
