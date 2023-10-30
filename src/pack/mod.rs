use std::collections::HashMap;

use crate::day::Day;

/// Denotes a single day pack (e.g. a single year)
#[derive(Debug)]
pub struct DayPack {
    pub name: String,
    pub days: HashMap<u8, Day>,
}

pub mod all_packs;
mod aoc23;
mod euler;
