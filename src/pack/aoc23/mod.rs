use std::collections::HashMap;
use crate::pack::DayPack;
mod day1;

pub fn get_pack() -> DayPack {
    DayPack {
       name: "aoc23".to_string(), 
       days: HashMap::from([
        (u8::from(1), day1::get_day("aoc23".to_string()))
        ])
    }
}