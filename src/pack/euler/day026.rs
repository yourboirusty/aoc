use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

fn get_cycle_length(n: u64) -> u64 {
    let mut idx = 0u64;
    let mut remainder = 1;
    let mut remainders = HashMap::new();
    while remainder != 0 {
        while remainder < n {
            remainder *= 10;
        }
        remainder %= n;
        if remainders.contains_key(&remainder) {
            return idx - remainders.get(&remainder).unwrap();
        }
        remainders.insert(remainder, idx);
        idx += 1;
    }
    0
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        (1..1000)
            .max_by_key(|n| get_cycle_length(*n))
            .unwrap()
            .to_string()
    }
}

get_day_fn!(Part1);

#[test]
fn test_get_cycle_length() {
    assert_eq!(get_cycle_length(2), 0);
    assert_eq!(get_cycle_length(3), 1);
    assert_eq!(get_cycle_length(7), 6);
}
