use std::{char, collections::HashMap, iter::Rev, str::Chars};

use crate::day::{Day, Solveable};

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let mut solution: u32 = 0;

        lines
            .iter()
            .map(|line| {
                let line_chars = line.chars();
                let mut calibration_value: u32 = 0;
                debug!("{line}");
                calibration_value += line_chars
                    .clone()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    * 10;
                calibration_value += line_chars
                    .clone()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                solution += calibration_value;
            })
            .count(); //Consume iterator

        solution.to_string()
    }
}
fn has_back(value: i32) -> bool {
    value % 10 != 0
}

#[test]
fn back_detected() {
    for no_back_val in [0, 10, 20, 30, 40] {
        assert!(!has_back(no_back_val));
    }
    for back_val in [1, 11, 25] {
        assert!(has_back(back_val));
    }
}

fn has_front(value: i32) -> bool {
    if value == 0 {
        return false;
    }
    (value / 10) > 0
}

#[test]
fn front_detected() {
    for no_front_val in [0, 1, 2, 3, 9] {
        assert!(!has_front(no_front_val));
    }
    for front_val in [10, 30, 70] {
        assert!(has_front(front_val));
    }
}

fn process_front(char_iter: &mut Chars<'_>) -> u32 {
    while let Some(val) = char_iter.next() {
        if val.is_ascii_digit() {
            return val.to_digit(10).unwrap();
        }
        let output = match val {
            'o' => check_for(char_iter, "one", 1),
            't' => {
                let two_check = check_for(char_iter, "two", 2);
                match two_check {
                    Some(x) => Some(x),
                    None => check_for(char_iter, "three", 3),
                }
            }
            'f' => {
                let four_check = check_for(char_iter, "four", 4);
                match four_check {
                    Some(x) => Some(x),
                    None => check_for(char_iter, "five", 5),
                }
            }
            's' => {
                let six_check = check_for(char_iter, "six", 6);
                match six_check {
                    Some(x) => Some(x),
                    None => check_for(char_iter, "seven", 7),
                }
            }
            'e' => check_for(char_iter, "eight", 8),
            'n' => check_for(char_iter, "nine", 9),
            _ => None,
        };
        if let Some(result) = output {
            return result;
        }
    }
    panic!("No results found! Irrecoverable blunder!");
}

fn check_for_multiple(char_iter: &mut Chars<'_>, words: &[&str]) -> Option<u32> {
    static mut CURRENT_WORDS: Vec<String> = Vec::new();
    let two_check = check_for(char_iter, "two", 2);
    match two_check {
        Some(x) => Some(x),
        None => check_for(char_iter, "three", 3),
    }
}

fn check_for(char_iter: &mut Chars<'_>, check_word: &'static str, value: u32) -> Option<u32> {
    let word_string = check_word.to_owned();
    let mut word_iter = word_string.chars();
    word_iter.next();
    let mut char_iter_copy = char_iter.to_owned();
    for letter in word_iter {
        let val = char_iter_copy.next()?;
        if val.is_ascii_digit() {
            return val.to_digit(10);
        }
        if val != letter {
            return None;
        }
    }

    Some(value)
}

fn process_back(char_iter: &mut Chars<'_>) -> u32 {
    while let Some(val) = char_iter.next() {
        if val.is_ascii_digit() {
            return val.to_digit(10).unwrap();
        }
        let output = match val {
            'e' => {
                let one_check = check_for(char_iter, "eno", 1);
                match one_check {
                    Some(x) => Some(x),
                    None => match check_for(char_iter, "eerht", 3) {
                        Some(x) => Some(x),
                        None => match check_for(char_iter, "evif", 5) {
                            Some(x) => Some(x),
                            None => check_for(char_iter, "enin", 9),
                        },
                    },
                }
            }
            'o' => check_for(char_iter, "owt", 2),
            'r' => check_for(char_iter, "ruof", 4),
            'x' => check_for(char_iter, "xis", 6),
            'n' => check_for(char_iter, "neves", 7),
            't' => check_for(char_iter, "thgie", 8),
            _ => None,
        };
        if let Some(result) = output {
            return result;
        }
    }
    panic!("No results found! Irrecoverable blunder!");
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        let mut solution: u32 = 0;

        lines.iter().for_each(|line| {
            let mut calibration_value = 0;

            let line_binding = line.clone();
            let mut line_chars_iter = line_binding.chars();
            let line_chars_rev_str: String = line_binding.chars().rev().collect();
            let mut line_chars_rev_iter = line_chars_rev_str.chars();
            calibration_value += 10 * process_front(&mut line_chars_iter);

            calibration_value += process_back(&mut line_chars_rev_iter);
            solution += calibration_value;
        });

        solution.to_string()
    }
}

get_day_fn!(Part1, Part2);
