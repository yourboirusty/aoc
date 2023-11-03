use num::BigInt;

use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let pow_thousand = BigInt::from(2).pow(1000);
        let ten = BigInt::from(10);
        let zero = BigInt::from(0);
        let mut n = pow_thousand;
        let mut ans: BigInt = 0.into();
        while &n > &zero {
            ans += &n % &ten;
            n /= &ten;
        }
        ans.to_string()
    }
}

get_day_fn!(Part1);
