use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        // assumed to be sorted
        let digits_of_interest = vec![1, 10, 100, 1000, 10000, 100000, 1000000];
        let mut digit_idx = 0;
        let mut i = 1;
        let mut digits_so_far = 0;
        let mut ans = 1;
        while digit_idx < digits_of_interest.len() {
            let digits_here = i.to_string().chars().collect::<Vec<_>>();
            while digit_idx < digits_of_interest.len()
                && digits_so_far + digits_here.len() >= digits_of_interest[digit_idx]
            {
                ans *= digits_here[digits_of_interest[digit_idx] - digits_so_far - 1]
                    .to_digit(10)
                    .unwrap();
                digit_idx += 1;
            }
            digits_so_far += digits_here.len();
            i += 1;
        }
        ans.to_string()
    }
}

get_day_fn!(Part1);
