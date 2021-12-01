use core::panic;

fn parse_input(input: &str, deck_size: usize) -> (usize, usize) {
    input.lines().fold((1, 0), |(v, c), line| {
        if line.starts_with("deal into new stack") {
            (deck_size - v, deck_size - c - 1)
        } else if line.starts_with("cut ") {
            if line.chars().nth(4).unwrap() == '-' {
                let value: usize = line[5..].parse().unwrap();
                (v, (c + value) % deck_size)
            } else {
                let value: usize = line[4..].parse().unwrap();
                (v, (c + deck_size - value) % deck_size)
            }
        } else if line.starts_with("deal with increment ") {
            let value: usize = line[20..].parse().unwrap();
            ((v * value) % deck_size, (c * value) % deck_size)
        } else {
            panic!("Unrecognized input: {}", line)
        }
    })
}

/*
pos 0 (1, 0)
new   (9, 9) = 9
new   (1, 0) = 0

pos 1 (1, 0)
new   (9, 9) = 18
new 
*/

// y - (ax + b)

fn safe_mul(a: usize, b: usize, n: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else if b == 1 {
        a
    } else if b & 1 == 1 {
        safe_mul((a + a) % n, b / 2, n) + a
    } else {
        safe_mul((a + a) % n, b / 2, n)
    }
}
fn safe_pow(a: usize, b: usize, n: usize) -> usize {
    if b == 0 {
        1
    } else if b == 1 {
        a
    } else {
        let a_mul = safe_mul(a, a, n);
        let part = safe_pow(a_mul, b / 2, n);
        if b & 1 == 1 {
            safe_mul(a, part, n)
        } else {
            part
        }
    }
}

fn mod_inverse(a: usize, b: usize) -> usize {
    let (mut old_r, mut r) = (a as i64, b as i64);
    let (mut old_s, mut s) = (1i64, 0i64);
    
    while r != 0 {
        let quotient = old_r / r;

        let new_r = old_r - quotient * r;
        old_r = r;
        r = new_r;

        let new_s = old_s - quotient * s;
        old_s = s;
        s = new_s;
    }

    if old_r != 1 {
        panic!("GCD(a, b) = GCD({}, {}) = {}", a, b, old_r);
    }
    assert!(old_s >= 0);
    old_s as usize
}

/// (x^0 + x^1 + ... + x^n) mod m
fn geom_sum(x: usize, n: usize, m: usize) -> usize {
    if x == 0 {
        1
    } else if x == 1 {
        (n + 1) % m
    } else {
        let divisor = x - 1;
        let mult = mod_inverse(divisor, m);

        let foo = {
            let tmp = safe_pow(x, n+1, m);
            if tmp == 0 {
                m - 1
            } else {
                tmp - 1
            }
        };

        safe_mul(foo, mult, m)
    }
}

fn geom_sum_naive(x: usize, n: usize, m: usize) -> usize {
    let mut sum = 1;
    let mut foo = 1;
    for _ in 1..n {
        foo = safe_mul(foo, x, m);
        sum = (sum + foo) % m;
    }
    sum
}

pub fn run(input: &str, start_position: usize, deck_size: usize, repetitions: usize) -> usize {
    let (v, c) = parse_input(input, deck_size);
    eprintln!("v: {}, c: {}", v, c);

    if v == 1 {
        (start_position + safe_mul(repetitions, c, deck_size)) % deck_size
    } else {
        let c_coef = geom_sum_naive(repetitions - 1, v, deck_size);

        let v_pow = safe_pow(v, repetitions, deck_size);
        //let a = (v_pow - 1) / (v - 1);
        //let b = safe_mul(c, a, deck_size);
        //let foo = c*((1-v_pow)/(1-v)) + v_pow * position;
        eprintln!("start: {}, end: {}", start_position, safe_mul(v_pow, start_position, deck_size) + safe_mul(c_coef, c, deck_size));
        eprintln!("{} * {} = {}, {} * {} = {}", v_pow, start_position, v_pow * start_position, c_coef, c, c * c_coef);
        (safe_mul(v_pow, start_position, deck_size) + safe_mul(c_coef, c, deck_size)) % deck_size
    }

    /*let mut position = start_position;
    let mut current = 0;
    while current < repetitions {
        if current % 10_000_000 == 0 {
            eprintln!("Iter: {}", current);
        }
        position = (safe_mul(position, v, deck_size) + c) % deck_size;
        current += 1;
    }
    position*/
}
