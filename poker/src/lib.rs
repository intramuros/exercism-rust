use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
}

fn convert(s: &str) -> (u8, &str) {
    let suit = s.get(s.len() - 1..).unwrap();
    let n = match &s[..s.len() - 1] {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        num => num.parse().unwrap(),
    };
    (n, suit)
}

impl From<&str> for Hand {
    fn from(cards: &str) -> Self {
        use HandType::*;

        let mut ranks = HashMap::new();
        let mut suits = HashSet::new();
        for (r, s) in cards.split_whitespace().map(convert) {
            *ranks.entry(r).or_insert(0) += 1;
            suits.insert(s);
        }

        let mut rank_sort: Vec<(&u8, &i32)> = ranks.iter().collect::<Vec<_>>();

        rank_sort.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => b.0.cmp(&a.0),
            ord => ord,
        });

        let rank_sort: Vec<_> = rank_sort.into_iter().map(|(c, _)| *c).collect();

        let suits_len = suits.len();

        match rank_sort.len() {
            5 if rank_sort
                .iter()
                .zip(rank_sort.iter().skip(1))
                .all(|(a, b)| a - b == 1) =>
            {
                match suits_len {
                    1 => Hand::new(StraightFlush, rank_sort),
                    _ => Hand::new(Straight, rank_sort),
                }
            }
            5 if rank_sort == vec![14, 5, 4, 3, 2] => match suits_len {
                1 => Hand::new(StraightFlush, vec![5, 4, 3, 2, 1]),
                _ => Hand::new(Straight, vec![5, 4, 3, 2, 1]),
            },
            5 => match suits_len {
                1 => Hand::new(Flush, rank_sort),
                _ => Hand::new(HighCard, rank_sort),
            },
            4 => Hand::new(OnePair, rank_sort),
            3 if ranks.values().any(|&val| val == 3) => Hand::new(ThreeOfAKind, rank_sort),
            3 => Hand::new(TwoPair, rank_sort),
            _ if ranks.values().any(|&val| val == 4) => Hand::new(FourOfAKind, rank_sort),
            _ => Hand::new(FullHouse, rank_sort),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    ranks: Vec<u8>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => Some(self.ranks.cmp(&other.ranks)),
            ord => Some(ord),
        }
    }
}

impl Hand {
    fn new(hand_type: HandType, ranks: Vec<u8>) -> Self {
        Hand { hand_type, ranks }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut winners = vec![];
    winners.push(hands[0]);
    let mut cur_winner = Hand::from(hands[0]);
    for hand in hands.iter().skip(1) {
        let h = Hand::from(*hand);
        if h > cur_winner {
            winners.clear();
            winners.push(hand);
            cur_winner = h;
        } else if h == cur_winner {
            winners.push(hand);
        }
    }
    Some(winners)
}
