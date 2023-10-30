use crate::pack::DayPack;
use std::collections::HashMap;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn get_pack() -> DayPack {
    let name = "euler".to_string();
    DayPack {
        name: name.clone(),
        days: HashMap::from([
            (u8::from(1), day1::get_day(name.clone())),
            (u8::from(2), day2::get_day(name.clone())),
            (u8::from(3), day3::get_day(name.clone())),
            (u8::from(4), day4::get_day(name.clone())),
            (u8::from(5), day5::get_day(name.clone())),
            (u8::from(6), day6::get_day(name.clone())),
        ]),
    }
}
