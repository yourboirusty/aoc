use std::collections::HashSet;

use crate::day::Day;
use crate::day::Solveable;

use super::shared::get_primes_under;
use super::shared::sum_of_divisors;

fn can_be_written(n: u64, abundant_numbers: &HashSet<u64>) -> bool {
    // note: this could be faster if I used a sorted vector in addition to the set
    // since I could break instead of continuing in the loop.
    for a in abundant_numbers {
        if a > &n {
            continue;
        }
        if abundant_numbers.contains(&(n - a)) {
            return true;
        }
    }
    false
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let primes = get_primes_under(28124);
        let abundant_numbers = (1..28124)
            .filter(|&n| n + n < sum_of_divisors(n, &primes))
            .collect::<HashSet<u64>>();
        (1..28124)
            .filter(|&n| !can_be_written(n, &abundant_numbers))
            .sum::<u64>()
            .to_string()
    }
}

get_day_fn!(Part1);

#[test]
fn test_can_be_written() {
    let primes = get_primes_under(28124);
    let abundant_numbers = (1..28124)
        .filter(|&n| n + n < sum_of_divisors(n, &primes))
        .collect::<HashSet<u64>>();
    assert!(can_be_written(24, &abundant_numbers));
    assert!(!can_be_written(25, &abundant_numbers));
}
