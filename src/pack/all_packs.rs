use std::collections::HashMap;

use crate::pack::DayPack;
use crate::pack::aoc23;

pub fn get_packs() -> HashMap<String, DayPack> {
    HashMap::from([
        (String::from("aoc23"), aoc23::get_pack())
    ])
}