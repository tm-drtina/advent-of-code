use std::str::FromStr;

fn transform(val: i64, subject: i64) -> i64 {
    (val * subject) % 20201227
}

fn batch_transform(subject: i64, i: i64) -> i64 {
    if i == 1 {
        return subject;
    }
    let sub_res = batch_transform(subject, i / 2);
    let res = (sub_res * sub_res) % 20201227;
    if i % 2 == 1 {
        (res * subject) % 20201227
    } else {
        res
    }
}

fn guess_private_key(pubkey1: i64, pubkey2: i64) -> (Option<i64>, Option<i64>) {
    let mut val1 = 1;
    let mut val2 = 1;
    for i in 1.. {
        val1 = transform(val1, 7);
        val2 = transform(val2, 7);

        if val1 == pubkey1 {
            return (Some(i), None);
        }
        if val2 == pubkey2 {
            return (None, Some(i));
        }
    }
    (None, None)
}

pub fn run(input: &str) -> i64 {
    let mut lines = input.lines();
    let pub1 = i64::from_str(lines.next().unwrap()).unwrap();
    let pub2 = i64::from_str(lines.next().unwrap()).unwrap();
    let priv_keys = guess_private_key(pub1, pub2);
    match priv_keys {
        (Some(priv_key), _) => batch_transform(pub2, priv_key),
        (_, Some(priv_key)) => batch_transform(pub1, priv_key),
        (None, None) => panic!("Private key not found!"),
    }
}
