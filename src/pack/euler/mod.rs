use crate::pack::DayPack;
use std::collections::HashMap;
mod shared;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day67;

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
            (u8::from(7), day7::get_day(name.clone())),
            (u8::from(8), day8::get_day(name.clone())),
            (u8::from(9), day9::get_day(name.clone())),
            (u8::from(10), day10::get_day(name.clone())),
            (u8::from(11), day11::get_day(name.clone())),
            (u8::from(12), day12::get_day(name.clone())),
            (u8::from(13), day13::get_day(name.clone())),
            (u8::from(14), day14::get_day(name.clone())),
            (u8::from(15), day15::get_day(name.clone())),
            (u8::from(16), day16::get_day(name.clone())),
            (u8::from(17), day17::get_day(name.clone())),
            (u8::from(18), day18::get_day(name.clone())),
            (u8::from(67), day67::get_day(name.clone())),
        ]),
    }
}
