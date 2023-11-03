use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        (1..1000)
            .filter(|n| n % 3 == 0 || n % 5 == 0)
            .sum::<u32>()
            .to_string()
    }
}

get_day_fn!(Part1);
