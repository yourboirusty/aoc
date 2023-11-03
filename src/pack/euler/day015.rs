use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        // Essentially we have 40 slots for movement, and we need to choose 20 of them to be down.
        // So we just need to compute 40 choose 20.
        let n: u64 = 40;
        let k: u64 = 20;
        (1u64..k + 1)
            .map(|i| (n - k + i) as f64 / i as f64)
            .product::<f64>()
            .round()
            .to_string()
    }
}

get_day_fn!(Part1);
