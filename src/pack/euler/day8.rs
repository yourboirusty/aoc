use std::cmp::max;

use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let numbers: Vec<u64> = lines
            .concat()
            .chars()
            .map(|c| c as u8 - b'0')
            .map(|n| n as u64)
            .collect();
        let mut product = numbers[0..13].iter().product::<u64>();
        let mut max_product = product;
        for i in 13..numbers.len() {
            if numbers[i - 13] == 0 {
                product = numbers[i - 12..i + 1].iter().product();
            } else {
                product = product / numbers[i - 13] * numbers[i];
            }
            max_product = max(product, max_product);
        }
        max_product.to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 8, vec![Box::new(Part1)])
}
