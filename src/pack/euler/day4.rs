use std::collections::HashSet;

use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let products = (100..1000)
            .flat_map(|i| (100..1000).map(move |j| i * j))
            .collect::<HashSet<u32>>();
        products
            .iter()
            .filter(|p| {
                let s = p.to_string();
                s == s.chars().rev().collect::<String>()
            })
            .max()
            .unwrap()
            .to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 4, vec![Box::new(Part1)])
}
