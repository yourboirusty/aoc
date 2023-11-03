use crate::day::Day;
use crate::day::Solveable;

use super::shared;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut n = 10000;
        let mut primes: Vec<u64>;
        loop {
            primes = shared::get_primes_under(n);
            if primes.len() >= 10_001 {
                break;
            }
            n *= 10;
        }
        primes[10_000].to_string()
    }
}

get_day_fn!(Part1);
