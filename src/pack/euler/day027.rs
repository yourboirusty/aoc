use crate::day::Day;
use crate::day::Solveable;

use super::shared::get_primes_under;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let primes = get_primes_under(10_000_000);
        let primes_set = primes.iter().collect::<std::collections::HashSet<_>>();
        let mut max = 0;
        let mut max_product = 0;
        for b in primes.iter().filter(|&n| n < &1000) {
            for a in -1000i64..1000 {
                let mut n = 0;
                loop {
                    let v = n * n + a * n + *b as i64;
                    if v > 10_000_000 {
                        panic!("Not enough primes")
                    }
                    if v < 0 || !primes_set.contains(&(v as u64)) {
                        break;
                    }
                    n += 1;
                }
                if n > max {
                    max = n;
                    max_product = a * *b as i64;
                }
            }
        }
        max_product.to_string()
    }
}

get_day_fn!(Part1);