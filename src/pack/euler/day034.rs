use crate::day::Day;
use crate::day::Solveable;

// Note: not eefficient as we could memoize and make it recursive, but
// for this task it doesn't matter.
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (1..n + 1).product()
    }
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let digit_factorials = (0..10).map(factorial).collect::<Vec<_>>();
        (3..1_000_000)
            .filter(|&n| {
                n == n
                    .to_string()
                    .chars()
                    .map(|c| digit_factorials[c.to_digit(10).unwrap() as usize])
                    .sum::<u64>()
            })
            .sum::<u64>()
            .to_string()
    }
}

get_day_fn!(Part1);
