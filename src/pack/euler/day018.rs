use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

fn get_predecessors(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if col > 0 {
        result.push((row - 1, col - 1));
    }
    if col < row {
        result.push((row - 1, col));
    }
    result
}

pub struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let triangle = lines
            .iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        let mut max_cost = HashMap::new();
        max_cost.insert((0, 0), triangle[0][0]);
        for row in 1..triangle.len() {
            for col in 0..triangle[row].len() {
                let max = get_predecessors(row, col)
                    .iter()
                    .map(|(r, c)| max_cost.get(&(*r, *c)).unwrap() + triangle[row][col])
                    .max()
                    .unwrap();
                max_cost.insert((row, col), max);
            }
        }
        max_cost
            .into_iter()
            .filter(|(k, _)| k.0 == triangle.len() - 1)
            .map(|(_, v)| v)
            .max()
            .unwrap()
            .to_string()
    }
}

get_day_fn!(Part1);
