use crate::day::Day;
use crate::day::Solveable;

fn find_equal_to_digit_powers(pow: u32) -> Vec<u32> {
    let digit_pows = (0u32..10).map(|n| n.pow(pow)).collect::<Vec<u32>>();
    (2u32..1000000)
        .filter(|n| {
            let mut sum = 0;
            let mut k = *n;
            while k > 0 {
                sum += digit_pows[(k % 10) as usize];
                k /= 10;
            }
            sum == *n
        })
        .collect()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        find_equal_to_digit_powers(5)
            .into_iter()
            .sum::<u32>()
            .to_string()
    }
}

get_day_fn!(Part1);

#[test]
fn test_find_equal_to_digit_powers() {
    assert_eq!(find_equal_to_digit_powers(4), vec![1634, 8208, 9474]);
}
