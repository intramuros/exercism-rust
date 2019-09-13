use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::{HashMap, HashSet};

const HIGH_CARD: u8 = 1;
const ONE_PAIR: u8 = 2;
const TWO_PAIR: u8 = 3;
const THREE_OF_A_KIND: u8 = 4;
const STRAIGHT: u8 = 5;
const FLUSH: u8 = 6;
const FULL_HOUSE: u8 = 7;
const FOUR_OF_A_KIND: u8 = 8;
const STRAIGHT_FLUSH: u8 = 9;

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

        match rank_sort.len() {
            5 => {
                if rank_sort
                    .iter()
                    .zip(rank_sort.iter().skip(1))
                    .all(|(a, b)| a - b == 1)
                {
                    match suits.len() {
                        1 => Hand::new(STRAIGHT_FLUSH, rank_sort),
                        _ => Hand::new(STRAIGHT, rank_sort),
                    }
                // check for ace low
                } else if rank_sort == vec![14, 5, 4, 3, 2] {
                    match suits.len() {
                        1 => Hand::new(STRAIGHT_FLUSH, vec![5, 4, 3, 2, 1]),
                        _ => Hand::new(STRAIGHT, vec![5, 4, 3, 2, 1]),
                    }
                } else {
                    match suits.len() {
                        1 => Hand::new(FLUSH, rank_sort),
                        _ => Hand::new(HIGH_CARD, rank_sort),
                    }
                }
            }
            4 => {
                // one pair
                Hand::new(ONE_PAIR, rank_sort)
            }
            3 => {
                if ranks.values().any(|&val| val == 3) {
                    // three of a kind
                    Hand::new(THREE_OF_A_KIND, rank_sort)
                } else {
                    // two pair
                    Hand::new(TWO_PAIR, rank_sort)
                }
            }
            _ => {
                if ranks.values().any(|&val| val == 4) {
                    // four of a kind
                    Hand::new(FOUR_OF_A_KIND, rank_sort)
                } else {
                    // full house
                    Hand::new(FULL_HOUSE, rank_sort)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: u8,
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
    fn new(hand_type: u8, ranks: Vec<u8>) -> Self {
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
