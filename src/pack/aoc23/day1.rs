use crate::day::Solveable;
use crate::day::Day;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        "Not implemented".to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &Vec<String>) -> String {
        "Not implemented".to_string()
    }
}

pub fn get_day(pack_name: String) -> Box<dyn Solveable + Sync> {
    Box::new(Day::new(pack_name, 1, Box::new(Part1), Box::new(Part2)))
}