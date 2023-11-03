use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut a = 1;
        let mut b = 2;
        let mut sum = 2;
        while a + b < 4000000 {
            let c = a + b;
            if c % 2 == 0 {
                sum += c;
            }
            a = b;
            b = c;
        }
        sum.to_string()
    }
}

get_day_fn!(Part1);
