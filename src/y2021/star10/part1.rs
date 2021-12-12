fn parse(s: &str) -> usize {
    let mut stack = Vec::<char>::new();
    for char in s.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' if stack.last() == Some(&'(') => {
                stack.pop().unwrap();
            }
            ')' => return 3,
            ']' if stack.last() == Some(&'[') => {
                stack.pop().unwrap();
            }
            ']' => return 57,
            '}' if stack.last() == Some(&'{') => {
                stack.pop().unwrap();
            }
            '}' => return 1197,
            '>' if stack.last() == Some(&'<') => {
                stack.pop().unwrap();
            }
            '>' => return 25137,
            _ => panic!("Unknown char"),
        }
    }
    0 // (in)complete, but valid
}

pub fn run(input: &str) -> usize {
    input.lines().map(parse).sum()
}
