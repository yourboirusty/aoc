use crate::day::Day;
use crate::day::Solveable;


struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        // It's just a copy of day18
        super::day18::Part1.solve(lines)
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 67, vec![Box::new(Part1)])
}
