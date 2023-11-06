use itertools::Itertools;

use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        "0123456789"
            .chars()
            .permutations(10)
            .nth(999_999)
            .unwrap()
            .iter()
            .join("")
    }
}

get_day_fn!(Part1);
