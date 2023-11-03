use std::collections::HashMap;

use crate::day::Day;
use crate::day::Solveable;

fn get_lens() -> (HashMap<u16, usize>, HashMap<u16, usize>, usize, usize) {
    let small_numbers: HashMap<u16, usize> = HashMap::from(
        [
            (0, "zero".len()),
            (1, "one".len()),
            (2, "two".len()),
            (3, "three".len()),
            (4, "four".len()),
            (5, "five".len()),
            (6, "six".len()),
            (7, "seven".len()),
            (8, "eight".len()),
            (9, "nine".len()),
            (10, "ten".len()),
            (11, "eleven".len()),
            (12, "twelve".len()),
            (13, "thirteen".len()),
            (14, "fourteen".len()),
            (15, "fifteen".len()),
            (16, "sixteen".len()),
            (17, "seventeen".len()),
            (18, "eighteen".len()),
            (19, "nineteen".len()),
        ]
    );
    
    let tens: HashMap<u16, usize> = HashMap::from(
        [
            (2, "twenty".len()),
            (3, "thirty".len()),
            (4, "forty".len()),
            (5, "fifty".len()),
            (6, "sixty".len()),
            (7, "seventy".len()),
            (8, "eighty".len()),
            (9, "ninety".len())
        ]
    );
    
    let hundred: usize = "hundred".len();
    
    let thousand: usize = "thousand".len();

    (small_numbers, tens, hundred, thousand)
}


struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let (small_numbers, tens, hundred, thousand) = get_lens();
        let mut ans = 0;
        for i in 1u16..1001 {
            let mut n = i;
            if i >= 1000 {
                ans += small_numbers[&(n / 1000)] + thousand;
                n %= 1000;
            }
            if n >= 100 {
                ans += small_numbers[&(n / 100)] + hundred;
                n %= 100;
                if n > 0 {
                    ans += 3; // "and"
                }
            }
            if n >= 20 {
                ans += tens[&(n / 10)];
                n %= 10;
            }
            if n > 0 {
                ans += small_numbers[&n];
            }
        }
        ans.to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 17, vec![Box::new(Part1)])
}
