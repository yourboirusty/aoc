use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        // assert all lines have the same length
        assert!(lines.iter().all(|l| l.len() == lines[0].len()));
        let grid = lines
            .iter()
            .map(|l| l.chars().map(|c| (c as u8 - b'0') as u64).collect())
            .collect::<Vec<Vec<u64>>>();
        let mut carry = 0;
        let mut result = Vec::new();
        for col in (0..grid[0].len()).rev() {
            let mut sum = carry;
            for row in 0..grid.len() {
                sum += grid[row][col];
            }
            result.push(sum % 10);
            carry = sum / 10;
        }
        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }
        result.reverse();

        result[0..10].iter().map(|n| n.to_string()).collect()
    }
}

get_day_fn!(Part1);
