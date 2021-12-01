fn new_stack(position: usize, deck_size: usize) -> usize {
    deck_size - position - 1
}

fn cut(value: usize, position: usize, deck_size: usize) -> usize {
    (position + (deck_size - value)) % deck_size
}

fn with_increment(value: usize, position: usize, deck_size: usize) -> usize {
    (value * position) % deck_size
}

fn run_part1(ops: &[Op], position: usize, deck_size: usize) -> usize {
    ops.iter().fold(position, |pos, op| match *op {
        Op::NewStack => new_stack(pos, deck_size),
        Op::Cut(value) => cut(value, pos, deck_size),
        Op::Inc(value) => with_increment(value, pos, deck_size),
    })
}

#[derive(Debug, Clone, Copy)]
enum Op {
    NewStack,
    Cut(usize),
    Inc(usize),
}

/*
start: 1        0
new:   *(-1)    -deck_size+1
cut:   --       + (deck_size - value)
inc:   * value  *value
*/

fn parse_input(input: &str, deck_size: usize) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("deal into new stack") {
                Op::NewStack
            } else if line.starts_with("cut ") {
                if line.chars().nth(4).unwrap() == '-' {
                    let value: usize = line[5..].parse().unwrap();
                    Op::Cut(deck_size - value)
                } else {
                    Op::Cut(line[4..].parse().unwrap())
                }
            } else if line.starts_with("deal with increment ") {
                Op::Inc(line[20..].parse().unwrap())
            } else {
                panic!("Unrecognized input: {}", line)
            }
        })
        .collect()
}

pub fn run(input: &str, start_position: usize, deck_size: usize, repetitions: usize) -> usize {
    let ops = parse_input(input, deck_size);
    let mut position = start_position;

    let mut current = 0;
    while current < repetitions {
        if current % 10_000_000 == 0 {
            eprintln!("Iter: {}", current);
        }
        position = run_part1(&ops, position, deck_size);
        if position == start_position {
            eprintln!("Found repetition at step: {}", current);
            panic!("Found repetition at step: {}", current);
        }
        current += 1;
    }

    position
}
