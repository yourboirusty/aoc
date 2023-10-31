use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let a = 600851475143;
        let primes = super::shared::get_primes_under((a as f64).sqrt().ceil() as u64);
        primes
            .iter()
            .filter(|&p| a % p == 0)
            .max()
            .unwrap()
            .to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 3, vec![Box::new(Part1)])
}
