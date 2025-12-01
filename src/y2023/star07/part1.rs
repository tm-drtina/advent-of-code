use std::str::FromStr;

use anyhow::{Result, anyhow};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five,      // 1 unique
    Four,      // 2 unique
    FullHouse, // 2 unique
    Three,     // 3 unique
    TwoPair,   // 3 unique
    OnePair,   // 4 unique
    HighCard,  // 5 unique
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<usize>,
    bid: usize,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (cards, bid) = s.split_once(' ').ok_or(anyhow!("Invalid format"))?;
        let bid = bid.parse()?;
        let cards = cards
            .chars()
            .map(|c| {
                CARDS
                    .iter()
                    .position(|c2| *c2 == c)
                    .ok_or(anyhow!("Invalid char"))
            })
            .collect::<Result<_>>()?;
        let mut counts = [0usize; 13];
        for &card in &cards {
            counts[card] += 1;
        }
        let hand_type = match (
            counts.iter().filter(|a| **a != 0).count(),
            counts.iter().max().unwrap(),
        ) {
            (5, _) => HandType::HighCard,
            (4, _) => HandType::OnePair,
            (3, 2) => HandType::TwoPair,
            (3, 3) => HandType::Three,
            (2, 3) => HandType::FullHouse,
            (2, 4) => HandType::Four,
            (1, _) => HandType::Five,
            _ => unreachable!(),
        };
        Ok(Self {
            hand_type,
            cards,
            bid,
        })
    }
}

pub fn run(input: &str) -> Result<usize> {
    let mut hands = input
        .lines()
        .map(str::parse::<Hand>)
        .collect::<Result<Vec<_>>>()?;
    hands.sort_unstable_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then(a.cards.cmp(&b.cards))
            .reverse()
    });
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum())
}
