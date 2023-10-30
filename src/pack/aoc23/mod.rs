use std::collections::HashMap;
use crate::pack::DayPack;
mod day1;

pub fn get_pack() -> DayPack {
    let name = "aoc23".to_string();
    DayPack {
       name: name.clone(),
       days: HashMap::from([
        (u8::from(1), day1::get_day(name))
        ])
    }
}