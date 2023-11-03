use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn score_name(name: &str, score_map: &HashMap<char, usize>) -> usize {
    name.chars().map(|c| score_map.get(&c).unwrap()).sum()
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let names = {
            let mut names = lines[0]
                .split(',')
                .map(|s| s.trim_matches('"'))
                .collect::<Vec<_>>();
            names.sort();
            names
        };
        let score_map = ALPHABET
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i + 1))
            .collect::<HashMap<_, _>>();
        names
            .iter()
            .enumerate()
            .map(|(i, n)| (i + 1) * score_name(n, &score_map))
            .sum::<usize>()
            .to_string()
    }
}

get_day_fn!(Part1);
