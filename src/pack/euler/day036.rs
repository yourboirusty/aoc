use crate::day::Day;
use crate::day::Solveable;

fn is_palindrome(s: String) -> bool {
    s.chars().rev().collect::<String>() == s
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        (1..1_000_000)
            .filter(|&n| is_palindrome(n.to_string()) && is_palindrome(format!("{:b}", n)))
            .sum::<u64>()
            .to_string()
    }
}

get_day_fn!(Part1);
