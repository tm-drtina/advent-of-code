
fn new_stack(position: usize, deck_size: usize) -> usize {
    deck_size - position - 1
}

fn cut(value: usize, position: usize, deck_size: usize) -> usize {
    if position >= value {
        position - value
    } else {
        deck_size - value + position
    }
}
fn cut_negative(value: usize, position: usize, deck_size: usize) -> usize {
    cut(deck_size - value, position, deck_size)
}

fn with_increment(value: usize, position: usize, deck_size: usize) -> usize {
    (value * position) % deck_size
}

pub fn run(input: &str, mut position: usize, deck_size: usize) -> usize {
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        position = if line.starts_with("deal into new stack") {
            new_stack(position, deck_size)
        } else if line.starts_with("cut ") {
            if line.chars().nth(4).unwrap() == '-' {
                cut_negative(line[5..].parse().unwrap(), position, deck_size)
            } else {
                cut(line[4..].parse().unwrap(), position, deck_size)
            }
        } else if line.starts_with("deal with increment ") {
            with_increment(line[20..].parse().unwrap(), position, deck_size)
        } else {
            panic!("Unrecognized input: {}", line)
        }
    }

    position
}
