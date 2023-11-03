use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        // It's just a copy of day18
        super::day018::Part1.solve(lines)
    }
}

get_day_fn!(Part1);
