use num::BigInt;

use crate::day::Day;
use crate::day::Solveable;

fn get_num_unique(n: u64) -> usize {
    (2..n + 1)
        .flat_map(|a| {
            (2..a + 1).map(move |b| {
                vec![BigInt::from(a).pow(b as u32), BigInt::from(b).pow(a as u32)].into_iter()
            })
        })
        .flatten()
        .collect::<std::collections::HashSet<_>>()
        .len()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        get_num_unique(100).to_string()
    }
}

get_day_fn!(Part1);

#[test]
fn test_get_num_unique() {
    assert_eq!(get_num_unique(5), 15);
}
