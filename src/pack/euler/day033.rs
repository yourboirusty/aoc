use std::collections::HashSet;

use crate::day::Day;
use crate::day::Solveable;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut num_product = 1;
        let mut denom_product = 1;
        for denominator in 10..100 {
            for numerator in 10..denominator {
                if denominator % 10 == 0 && numerator % 10 == 0 {
                    continue;
                }
                let numerator_chars = numerator.to_string().chars().collect::<HashSet<char>>();
                let denominator_chars = denominator.to_string().chars().collect::<HashSet<char>>();

                for common_char in numerator_chars.intersection(&denominator_chars) {
                    let numerator_without_char = numerator.to_string().replace(*common_char, "");
                    let denominator_without_char =
                        denominator.to_string().replace(*common_char, "");
                    if numerator_without_char == "" || denominator_without_char == "" {
                        continue;
                    }
                    let numerator_without_char = numerator_without_char.parse::<u64>().unwrap();
                    let denominator_without_char = denominator_without_char.parse::<u64>().unwrap();
                    if numerator_without_char as f32 / denominator_without_char as f32
                        == numerator as f32 / denominator as f32
                    {
                        num_product *= numerator_without_char;
                        denom_product *= denominator_without_char;
                    }
                }
            }
        }
        let gcd = gcd(num_product, denom_product);
        denom_product /= gcd;
        denom_product.to_string()
    }
}

get_day_fn!(Part1);
