use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        for a in 1..1000 {
            for b in a+1..1000 {
                let c = 1000 - a - b;
                if c <= b {
                    break;
                }
                if a * a + b * b == c * c {
                    return (a * b * c).to_string();
                }
            }
        }
        "Not found".to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 9, vec![Box::new(Part1)])
}
