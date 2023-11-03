use crate::day::Day;
use crate::day::Solveable;

const DAYS_IN_MONTH: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_year_leap(year: usize) -> bool {
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}

#[memoize::memoize]
fn get_in_year(start_dow: usize, leap: bool) -> usize {
    let mut dow = start_dow;
    let mut ans = 0;
    for (i, &days) in DAYS_IN_MONTH.iter().enumerate() {
        if dow == 6 {
            ans += 1;
        }
        let leap_day = if leap && i == 1 { 1 } else { 0 };
        dow = (dow + days + leap_day) % 7;
    }
    ans
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        // Jan 1 1900 was a Monday
        let mut start_dow = 0;
        let mut ans = 0;
        for year in 1901..2001 {
            start_dow = (start_dow + if is_year_leap(year - 1) { 366 } else { 365 }) % 7;
            ans += get_in_year(start_dow, is_year_leap(year));
        }
        ans.to_string()
    }
}

get_day_fn!(Part1);
