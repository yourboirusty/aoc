use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        super::shared::get_primes_under(2_000_000)
            .iter()
            .sum::<u64>()
            .to_string()
    }
}

get_day_fn!(Part1);
