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


pub fn get_day(pack_name: String) -> Day {
    Day::new(
        pack_name,
        1,
        vec![Box::new(Part1)],
    )
}
