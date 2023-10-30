use crate::day::Day;
use crate::day::Solveable;

fn gcd (a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut a = 1;
        for i in 1..=20 {
            a = a * i / gcd(a, i);
        }
        a.to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 5, vec![Box::new(Part1)])
}
