use std::collections::HashSet;

use crate::day::Day;
use crate::day::Solveable;

fn is_valid(digits: &String) -> bool {
    digits.len() == 9 && digits.chars().collect::<HashSet<_>>().len() == 9 && !digits.contains("0")
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        (1..100000)
            .map(|n| {
                let mut concat: String = String::new();
                let mut i = 1;
                while concat.len() < 9 {
                    concat.push_str(&(n * i).to_string());
                    i += 1;
                }
                if is_valid(&concat) {
                    Some(concat.parse::<u64>().unwrap())
                } else {
                    None
                }
            })
            .filter(|n| n.is_some())
            .map(|n| n.unwrap())
            .max()
            .unwrap()
            .to_string()
    }
}

get_day_fn!(Part1);
