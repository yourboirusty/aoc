use std::{cmp::Ordering, collections::HashMap, iter::zip, str::FromStr};

use base::main_day_fn;
use phf::phf_map;

type Kind = &'static u16;

type Cards = [Kind; 5];

const CARD_KIND_MAP: phf::Map<char, u16> = phf_map! {
    '2' => 1,
    '3' => 2,
    '4' => 3,
    '5' => 4,
    '6' => 5,
    '7' => 6,
    '8' => 7,
    '9' => 8,
    'T' => 9,
    'J' => 10,
    'Q' => 11,
    'K' => 12,
    'A' => 13,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl From<&Cards> for HandType {
    fn from(value: &Cards) -> Self {
        let mut card_set: HashMap<&Kind, u8> = HashMap::with_capacity(5);
        for card in value {
            let insert_value = card_set.get(card).unwrap_or(&0_u8);
            card_set.insert(card, insert_value + 1);
        }
        debug!("cards {:?}", value);
        debug!("  set {:?}", card_set);

        if let Some(value) = parse_hand_type(card_set.clone()) {
            debug!("     {:?}\n", value);
            return value;
        }
        panic!("Unknown hand type");
    }
}

fn parse_hand_type(card_set: HashMap<&&u16, u8>) -> Option<HandType> {
    let mut card_counts = card_set.into_values().collect::<Vec<u8>>();
    card_counts.sort();
    card_counts.reverse();
    trace!("{:?}", card_counts);
    match card_counts.as_slice() {
        [5] => Some(HandType::FiveKind),
        [4, 1] => Some(HandType::FourKind),
        [3, 2] => Some(HandType::FullHouse),
        [3, 1, 1] => Some(HandType::ThreeKind),
        [2, 2, 1] => Some(HandType::TwoPair),
        [2, 1, 1, 1] => Some(HandType::Pair),
        _ => Some(HandType::HighCard),
    }
}

#[derive(Debug)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
    bid: u32,
}

impl FromStr for Hand {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').unwrap();
        let bid = bid_str.parse::<u32>().unwrap();
        let cards: Cards = cards_str
            .bytes()
            .map(|c| CARD_KIND_MAP.get(&(c as char)).unwrap())
            .collect::<Vec<Kind>>()
            .try_into()
            .unwrap();
        let hand_type = HandType::from(&cards);

        Ok(Hand {
            cards,
            hand_type,
            bid,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for (&my_card, &other_card) in zip(self.cards, other.cards) {
            if my_card != other_card {
                return false;
            }
        }
        info!("Bingo {:?}", self);
        self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }
        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }
        for (&my_card, &other_card) in zip(self.cards, other.cards) {
            if my_card > other_card {
                return Some(Ordering::Greater);
            }
            if my_card < other_card {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn test_hand_cmp() {
    let equal = Hand::from_str("AAAAA 2").unwrap();
    assert!(equal == equal);
    let not_equal = Hand::from_str("KKKKK 3").unwrap();
    assert!(equal != not_equal);
    for (smaller_str, bigger_str) in [
        ("2AAA3", "3AAA2"),
        ("3AA22", "2AAAA"),
        ("2AAAA", "33332"),
        ("77788", "77888"),
    ] {
        let smaller = Hand::from_str(format!("{} 483", smaller_str).as_str()).unwrap();
        let bigger = Hand::from_str(format!("{} 483", bigger_str).as_str()).unwrap();
        assert!(bigger != smaller);
        assert!(bigger > smaller);
        assert!(!(bigger < smaller));
        assert!(smaller < bigger);
        assert!(!(smaller > bigger));
    }
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let mut hands: Vec<Hand> = lines
            .iter()
            .map(|line| Hand::from_str(line.as_str()).unwrap())
            .collect();
        hands.sort();
        let winnings: usize = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| {
                let rank = idx + 1;
                let winnings = rank * hand.bid as usize;
                debug!(
                    "Rank {rank} {:?} bid {}, won {winnings}",
                    hand.hand_type, hand.bid
                );
                winnings
            })
            .sum();

        winnings.to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        todo!();
    }
}

main_day_fn!(Part1, Part2);
