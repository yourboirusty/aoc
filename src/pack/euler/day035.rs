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
                let mut p = p;
                let mut rotations = Vec::new();
                for _ in 0..p.to_string().len() {
                    rotations.push(p);
                    p = p / 10 + (p % 10) * 10u64.pow(p.to_string().len() as u32 - 1);
                }
                rotations.into_iter().all(|r| primes_set.contains(&r))
            })
            .count()
            .to_string()
    }
}

get_day_fn!(Part1);
