use std::{collections::HashSet, str::FromStr};

use base::main_day_fn;
use regex::{Error, Regex};

#[derive(Clone, Copy, Debug)]
struct Card {
    id: u32,
    value: u32,
    matches: u32,
    multiplier: u32,
}

impl FromStr for Card {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Card\s+(?P<id>\d+):\s+(?P<winning>(?:\d+\s+)+)\|[ \s]+(?P<scratched>(?:\d+?[ \s]*)+)",
        )
        .unwrap();

        let cap = re.captures(s).unwrap();

        let id: u32 = cap.name("id").unwrap().as_str().parse().unwrap();
        let scratched: HashSet<u32> = cap
            .name("scratched")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();
        let winning: HashSet<u32> = cap
            .name("winning")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        let intersection_count = winning.intersection(&scratched).count() as u32;
        let value = if intersection_count == 0 {
            0
        } else {
            2_u32.pow(intersection_count - 1)
        };
        Ok(Card {
            id,
            value,
            matches: intersection_count,
            multiplier: 1,
        })
    }
}

struct Part1;

impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let cards = lines.iter().map(|line| Card::from_str(line).unwrap());
        let value_sum: u32 = cards.map(|card| card.value).sum();
        value_sum.to_string()
    }
}

struct Part2;

impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        let mut cards: Vec<Card> = lines
            .iter()
            .map(|line| Card::from_str(line).unwrap())
            .collect();
        let mut value_sum = 0;
        let card_count = cards.len();
        for i in 0..card_count {
            let card = cards[i].to_owned();
            value_sum += card.multiplier;
            debug!("{:?}", card);
            if card.matches == 0 {
                debug!("Card {} 0 matches", i + 1);
                continue;
            }
            for idx in card.id..card.id + card.matches {
                debug!("Loading {idx}");
                if idx as usize >= card_count {
                    debug!("Overflow!");
                    break;
                }
                cards[idx as usize].multiplier += card.multiplier;
                debug!("Upgraded {:?}", cards[idx as usize]);
            }
        }
        debug!("{:?}", cards);
        value_sum.to_string()
    }
}
main_day_fn!(Part1, Part2);
