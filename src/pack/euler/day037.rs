use crate::day::Day;
use crate::day::Solveable;

use super::shared::get_primes_under;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let primes = get_primes_under(1_000_000);
        let primes_set = primes.iter().collect::<std::collections::HashSet<_>>();

        primes
            .clone()
            .into_iter()
            .filter(|&p| {
                if p < 10 {
                    return false;
                }
                let mut trunc_left = Vec::new();
                let mut trunc_right = Vec::new();
                for i in 0..p.to_string().len() {
                    let ten = 10u64.pow(i as u32);
                    if ten > 1 {
                        trunc_left.push(p % ten);
                    }
                    if ten < p {
                        trunc_right.push(p / ten);
                    }
                }
                trunc_left.into_iter().all(|r| primes_set.contains(&r))
                    && trunc_right.into_iter().all(|r| primes_set.contains(&r))
            })
            .sum::<u64>()
            .to_string()
    }
}

get_day_fn!(Part1);
