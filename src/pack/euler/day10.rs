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

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 10, vec![Box::new(Part1)])
}
