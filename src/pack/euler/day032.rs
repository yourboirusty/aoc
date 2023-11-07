use std::collections::HashSet;

use crate::day::Day;
use crate::day::Solveable;

fn get_unique_digits(n: u64) -> HashSet<char> {
    n.to_string().chars().collect::<HashSet<char>>()
}

fn is_valid(chars: &HashSet<char>, num_digits: usize) -> bool {
    chars.len() == num_digits && !chars.contains(&'0')
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut ans = 0;
        // So we want the product, the multiplier, and the multiplicand,to have 9 digtits total
        // and also the product of m-digit and n-digit numbers is either m + n or m + n - 1 digits
        // So the following length combinations are possible:
        // 1 * 4 = 4
        // 2 * 3 = 4
        // So we're only considering 4-digit numbers as the result
        for result in 1000..10000 {
            let result_set = get_unique_digits(result);
            if !is_valid(&result_set, 4) {
                continue;
            }
            for multiplier in 1..((result as f32).sqrt().ceil() as u64) {
                if result % multiplier != 0 {
                    continue;
                }
                let multiplier_set = get_unique_digits(multiplier);
                if !is_valid(&multiplier_set, multiplier.to_string().len()) {
                    continue;
                }
                let multiplicand = result / multiplier;
                let multiplicand_set = get_unique_digits(multiplicand);
                if !is_valid(&multiplicand_set, multiplicand.to_string().len()) {
                    continue;
                }
                let digit_count = result_set
                    .union(&multiplier_set)
                    .cloned()
                    .collect::<HashSet<char>>()
                    .union(&multiplicand_set)
                    .count();
                if digit_count == 9 {
                    ans += result;
                    break;
                }
            }
        }
        ans.to_string()
    }
}

get_day_fn!(Part1);
