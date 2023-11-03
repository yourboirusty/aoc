use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

use super::shared::get_primes_under;

fn get_proper_divisors(n: u64, primes: &Vec<u64>) -> Vec<u64> {
    let mut ans = vec![1];
    let mut d = n;
    for p in primes {
        if p >= &n {
            break;
        }
        if d % p != 0 {
            continue;
        }
        let mut new_ans = ans.clone();
        let mut p_accum = 1;
        while d % p == 0 {
            p_accum *= p;
            d /= p;
            for a in &ans {
                if a * p_accum >= n {
                    continue;
                }
                new_ans.push(a * p_accum);
            }
        }
        ans = new_ans;
    }
    ans.sort();
    ans
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let primes = get_primes_under(10000);
        // map from n to sum of proper divisors of n
        let sum_proper_divisors = (2..10000)
            .map(|n| (n, get_proper_divisors(n, &primes).iter().sum()))
            .collect::<HashMap<u64, u64>>();
        sum_proper_divisors
            .iter()
            .filter(|(&n, &sum)| {
                let other_sum = sum_proper_divisors.get(&sum).unwrap_or(&0);
                n != sum && n == *other_sum
            })
            .map(|(n, _)| n)
            .sum::<u64>()
            .to_string()
    }
}

get_day_fn!(Part1);

#[test]
fn test_get_proper_divisors() {
    let primes = get_primes_under(10000);
    assert_eq!(
        get_proper_divisors(220, &primes),
        vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]
    );
    assert_eq!(get_proper_divisors(284, &primes), vec![1, 2, 4, 71, 142]);
}
