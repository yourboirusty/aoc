
use memoize::memoize;

use crate::day::Day;
use crate::day::Solveable;

#[memoize]
fn find_ways_to_sum(sum: usize, accum: u64, denominations: Vec<usize>) -> u64 {
    if denominations.is_empty() {
        return if sum == 0 { accum + 1 } else { 0 };
    }
    let mut ways = accum;
    let mut sum = sum;
    ways += find_ways_to_sum(sum, accum, denominations[1..].to_vec());
    while sum >= denominations[0] {
        ways += find_ways_to_sum(
            sum - denominations[0],
            accum,
            denominations[1..].to_vec(),
        );
        sum -= denominations[0];
    }
    ways
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        find_ways_to_sum(1000, 0, vec![1, 2, 5, 10, 20, 50, 100, 200]).to_string()
    }
}

get_day_fn!(Part1);

