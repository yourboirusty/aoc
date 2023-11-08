use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut max = 0;
        let mut max_p = 0;
        for p in 1..1000 {
            let mut ans = 0;
            for a in 1..p {
                for b in 1..a {
                    let c = p - a - b;
                    if c <= 0 {
                        break;
                    }
                    if a * a + b * b == c * c {
                        ans += 1;
                    }
                }
            }
            if ans > max {
                max = ans;
                max_p = p;
            }
        }
        max_p.to_string()
    }
}

get_day_fn!(Part1);
