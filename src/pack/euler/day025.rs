use num::BigInt;

use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let mut a: BigInt = 1.into();
        let mut b: BigInt = 1.into();
        let mut idx = 2;
        while b.to_string().len() < 1000 {
            let tmp = b.clone();
            b += &a;
            a = tmp;
            idx += 1;
        }
        idx.to_string()
    }
}

get_day_fn!(Part1);
