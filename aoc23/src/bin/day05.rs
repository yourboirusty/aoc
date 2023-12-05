use std::str::FromStr;

use base::{Day, Solveable};

#[macro_use]
extern crate log;

#[derive(Debug, Clone)]
struct AlmanacMapping {
    source_start: u32,
    range: u32,
    destination_start: u32,
}

impl AlmanacMapping {
    fn transform(&self, val: &usize) -> Option<usize> {
        if *val >= self.source_start as usize
            && *val - self.source_start as usize <= self.range as usize
        {
            return Some(*val - self.source_start as usize + self.destination_start as usize);
        }
        None
    }

    //Returns mappings sorted by source_start
    fn parse_mappings(iter: &mut dyn Iterator<Item = &String>) -> Vec<Self> {
        let mut almanac_mappings = Vec::new();

        let Some(header) = iter.next() else {
            panic!("Irrecoverable blunder! The iter is completely empty!")
        };

        trace!("{header}");
        let (source_str, destination_str) = header
            .split_once(' ')
            .unwrap()
            .0
            .split_once("-to-")
            .unwrap();

        for line in iter {
            if line.is_empty() {
                break;
            }
            let mut data: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let range = data.pop().unwrap();
            let source_start = data.pop().unwrap();
            let destination_start = data.pop().unwrap();

            trace!("{line}");
            almanac_mappings.push(AlmanacMapping {
                source_start,
                range,
                destination_start,
            })
        }

        almanac_mappings.sort_unstable_by_key(|k| k.source_start);
        debug!("{source_str} to {destination_str} {:?}", almanac_mappings);
        almanac_mappings
    }
}

fn parse_file(lines: &[String]) -> (Vec<Vec<AlmanacMapping>>, Vec<usize>) {
    let mut iter = lines.iter().peekable();
    let seeds: Vec<usize> = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    iter.next();

    let mut almanac_mappings: Vec<Vec<AlmanacMapping>> = Vec::new();

    loop {
        almanac_mappings.push(AlmanacMapping::parse_mappings(&mut iter));
        if iter.peek().is_none() {
            break;
        }
    }

    (almanac_mappings, seeds)
}

fn process_seed(
    seed: &mut usize,
    mapping: &mut AlmanacMapping,
    mapping_iter: &mut dyn Iterator<Item = &AlmanacMapping>,
) {
    if *seed < mapping.source_start as usize {
        debug!("{seed} too shmol");
        return;
    }
    if let Some(transform) = mapping.transform(seed) {
        debug!("{seed} just right");
        *seed = transform;
        return;
    }
    if let Some(new_mapping) = mapping_iter.next() {
        debug!("{seed} too large, checking next mapping");
        *mapping = new_mapping.to_owned();
        process_seed(seed, mapping, mapping_iter);
    }
}

struct Part1;

impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let (mappings, mut seeds) = parse_file(lines);
        mappings.iter().for_each(|mapping_set| {
            seeds.iter_mut().for_each(|seed| {
                let mut mapping_iter = mapping_set.iter();
                let mut current_mapping = mapping_iter.next().unwrap().to_owned();
                process_seed(seed, &mut current_mapping, &mut mapping_iter)
            });
        });
        debug!("{:?}", seeds);
        seeds.iter().min().unwrap().to_string()
    }
}

struct Part2;

impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        let (mappings, seed_ranges) = parse_file(lines);

        info!("Generating seeds");
        let mut seeds: Vec<usize> = seed_ranges
            .chunks(2)
            .flat_map(|chunk| {
                let range_start = chunk[0];
                let range = chunk[1];
                (range_start..range_start + range).collect::<Vec<usize>>()
            })
            .collect();
        info!("Starting to process {} seeds", seeds.len());
        mappings.into_iter().for_each(|mapping_set| {
            let mut mapping_iter = mapping_set.iter();
            let mut current_mapping = mapping_iter.next().unwrap().to_owned();
            seeds
                .iter_mut()
                .for_each(|seed| process_seed(seed, &mut current_mapping, &mut mapping_iter));
            info!("Layer done");
        });
        debug!("{:?}", seeds);
        seeds.iter().min().unwrap().to_string()
    }
}
fn main() {
    pretty_env_logger::init_timed();
    let file_str = file!().to_string();
    let day_name = file_str
        .rsplit_once('/')
        .unwrap()
        .1
        .rsplit_once('.')
        .unwrap()
        .0;
    let root_folder = file_str.rsplit_once("src").unwrap().0;
    let path = format!("{root_folder}input/{day_name}.txt");
    let day_obj = Day::new(day_name.to_string(), vec![Box::new(Part1), Box::new(Part2)]);

    day_obj.solve_standalone(&path);
}
