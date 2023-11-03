use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut collatz: HashMap<u64, u64> = HashMap::new();
        collatz.insert(1, 1);
        for i in 2u64..1_000_000 {
            let mut chain = vec![i];
            while !collatz.contains_key(&chain[chain.len() - 1]) {
                let last = &chain[chain.len() - 1];
                if last % 2 == 0 {
                    chain.push(last / 2);
                } else {
                    chain.push(3 * last + 1);
                }
            }
            let chain_len = collatz.get(&chain[chain.len() - 1]).copied().unwrap();
            for (idx, &n) in chain.iter().enumerate() {
                collatz.insert(n, chain_len + (chain.len() - idx) as u64);
            }
        }

        collatz
            .iter()
            .filter(|(&k, _)| k < 1_000_000)
            .max_by_key(|(_, &v)| v)
            .unwrap()
            .0
            .to_string()
    }
}

get_day_fn!(Part1);
