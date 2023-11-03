use num::BigInt;

use crate::day::Day;
use crate::day::Solveable;

fn factorial(n: u64) -> BigInt {
    (1..n + 1).map(BigInt::from).product()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        factorial(100)
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .sum::<u64>()
            .to_string()
    }
}

get_day_fn!(Part1);
