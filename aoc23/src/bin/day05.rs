use std::{cmp::Ordering, ops::Range};

use base::main_day_fn;

#[derive(Debug, Clone)]
struct AlmanacMapping {
    source_start: usize,
    destination_str: String,
    range: usize,
    destination_start: usize,
}

impl AlmanacMapping {
    fn transform(&self, val: &usize) -> Option<usize> {
        if *val >= self.source_start && *val - self.source_start <= self.range {
            debug!(
                "Transforming {} source {} destination {} range {}",
                val, self.source_start, self.destination_start, self.range
            );
            return Some(*val - self.source_start + self.destination_start);
        }
        None
    }

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
            let mut data: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let range = data.pop().unwrap();
            let source_start = data.pop().unwrap();
            let destination_start = data.pop().unwrap();

            trace!("{line}");
            almanac_mappings.push(AlmanacMapping {
                destination_str: destination_str.to_owned(),
                source_start,
                range,
                destination_start,
            })
        }

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
    seed: &usize,
    mapping: &mut AlmanacMapping,
    mapping_iter: &mut dyn Iterator<Item = &AlmanacMapping>,
) -> Option<usize> {
    if let Some(transform) = mapping.transform(seed) {
        debug!("{seed} just right, choosing {transform}");
        return Some(transform);
    }

    if let Some(new_mapping) = mapping_iter.next() {
        trace!("{seed} too large, checking next mapping");
        *mapping = new_mapping.to_owned();
        return process_seed(seed, mapping, mapping_iter);
    }
    None
}

struct Part1;

impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let (mappings, mut seeds) = parse_file(lines);
        mappings.iter().for_each(|mapping_set| {
            seeds.iter_mut().for_each(|seed| {
                let mut mapping_iter = mapping_set.iter();
                let mut current_mapping = mapping_iter.next().unwrap().to_owned();
                if let Some(new_seed) = process_seed(seed, &mut current_mapping, &mut mapping_iter)
                {
                    *seed = new_seed;
                }
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

        info!("Generating seed ranges");
        let seeds: Vec<Range<usize>> = seed_ranges
            .chunks(2)
            .map(|chunk| {
                let range_start = chunk[0];
                let range = chunk[1];
                range_start..range_start + range
            })
            .collect();
        info!("Starting to process {} seeds", seeds.len());
        let results: Vec<usize> = seeds
            .iter()
            .flat_map(|seed_range| {
                seed_range.to_owned().map(|seed| {
                    debug!("Running seed {seed}");
                    let mut final_seed = seed.to_owned();
                    mappings.iter().for_each(|mapping_set| {
                        debug!("Mapping for {}", mapping_set[0].destination_str);
                        if let Ok(target_mapping) = mapping_set.binary_search_by(|map| {
                            if final_seed < map.source_start {
                                return Ordering::Less;
                            };
                            if final_seed >= map.source_start
                                && final_seed - map.source_start <= map.range
                            {
                                return Ordering::Equal;
                            };
                            Ordering::Greater
                        }) {
                            final_seed =
                                mapping_set[target_mapping].transform(&final_seed).unwrap();
                        };
                    });
                    debug!("Final seed {final_seed}");
                    final_seed
                })
            })
            .collect();

        debug!("Results {:?}", results);
        results.iter().min().unwrap().to_string()
    }
}

main_day_fn!(Part1, Part2);
