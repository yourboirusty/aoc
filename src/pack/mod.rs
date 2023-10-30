use std::collections::HashMap;

use crate::day::Solveable;

/// Denotes a single day pack (e.g. a single year)
#[derive(Debug)]
pub struct DayPack {
    pub name: String,
    pub days: HashMap<u8, Box<dyn Solveable + Sync>>,
}

mod aoc23;
pub mod all_packs;