use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        "Not implemented".to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        "Not implemented".to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(
        pack_name,
        1,
        vec![Box::new(Part1), Box::new(Part2)],
    )
}
