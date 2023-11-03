use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

const MAX_SEARCH: u64 = 70_000;

fn num_divisors(primes: &Vec<u64>, n: u64) -> u64 {
    let mut prime_divisors: HashMap<u64, u64> = HashMap::new();
    let mut idx = 0;
    let mut n = n;
    while n > 1 {
        if idx >= primes.len() {
            panic!("Not enough primes");
        }
        let prime = primes[idx];
        if n % prime == 0 {
            let count = prime_divisors.get(&prime).unwrap_or(&0) + 1;
            prime_divisors.insert(prime, count);
            n /= prime;
        } else {
            idx += 1;
        }
    }
    prime_divisors.values().map(|&v| v + 1).product()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let primes = super::shared::get_primes_under(MAX_SEARCH);
        let mut n = 1;
        let mut tn = 1;
        loop {
            let num_div = num_divisors(&primes, tn);
            if num_div > 500 {
                break;
            }
            n += 1;
            tn += n;
        }
        tn.to_string()
    }
}

get_day_fn!(Part1);
