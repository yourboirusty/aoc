use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let a = (1..101).map(|x| x * x).sum::<u32>();
        let b = (1..101).sum::<u32>().pow(2);
        (b - a).to_string()
    }
}

get_day_fn!(Part1);
