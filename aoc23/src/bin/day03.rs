use std::collections::HashMap;

use base::{Day, Solveable};

#[macro_use]
extern crate log;

struct Part1;

fn check_around(data: &Vec<&[u8]>, x: usize, y: usize) -> Option<((usize, usize), char)> {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in directions.iter() {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0
            && new_x < data.len() as isize
            && new_y >= 0
            && new_y < data[new_x as usize].len() as isize
        {
            let character = data[new_x as usize][new_y as usize] as char;

            if !character.is_ascii_digit() && character != '.' {
                return Some(((new_x as usize, new_y as usize), character));
            }
        }
    }

    None
}

impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let full_schematic: Vec<&[u8]> = lines.iter().map(|line| line.as_bytes()).collect();
        let sum: u32 = lines
            .iter()
            .enumerate()
            .map(|(x_idx, line)| sum_line(&x_idx, line, &full_schematic))
            .sum();
        sum.to_string()
    }
}

fn sum_line(x_idx: &usize, line: &str, full_schematic: &Vec<&[u8]>) -> u32 {
    let mut found_digits: Vec<u32> = Vec::new();
    let mut part_number_flag = false;
    let mut line_sum: u32 = 0;
    debug!("{line}");

    for (y_idx, c) in line.bytes().enumerate() {
        let cc = c as char;
        trace!("looking at {cc}");
        if !cc.is_ascii_digit() {
            if part_number_flag {
                let found_number = found_digits
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(idx, dig)| dig * (10_u32.pow(idx as u32)))
                    .sum::<u32>();

                line_sum += found_number;
                part_number_flag = false;
                debug!("found {found_number}");
            }
            trace!("Looking at {cc}, discarded");
            found_digits.clear();
            continue;
        }

        found_digits.push(cc.to_digit(10).unwrap());
        debug!("Found {:?}", found_digits);

        if check_around(full_schematic, x_idx.to_owned(), y_idx).is_some() {
            debug!("Found a symbol");
            part_number_flag = true;
            continue;
        }
    }
    if part_number_flag {
        line_sum += found_digits
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, dig)| dig * (10_u32.pow(idx as u32)))
            .sum::<u32>();
    }
    line_sum
}

type GearMap = HashMap<(usize, usize), Vec<u32>>;

fn find_potential_gears(
    x_idx: &usize,
    line: &str,
    full_schematic: &Vec<&[u8]>,
    gears: &mut GearMap,
) {
    let mut found_digits: Vec<u32> = Vec::new();
    let mut part_number_flag: Option<(usize, usize)> = None;
    debug!("{line}");

    for (y_idx, c) in line.bytes().enumerate() {
        let cc = c as char;
        trace!("looking at {cc}");
        if !cc.is_ascii_digit() {
            if let Some(cog_location) = part_number_flag {
                let found_number = found_digits
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(idx, dig)| dig * (10_u32.pow(idx as u32)))
                    .sum::<u32>();

                part_number_flag = None;

                gears.entry(cog_location).or_default().push(found_number);
                debug!("found {found_number}");
            }
            trace!("Looking at {cc}, discarded");
            found_digits.clear();
            continue;
        }

        found_digits.push(cc.to_digit(10).unwrap());
        debug!("Found {:?}", found_digits);

        if let Some((cog_location, '*')) = check_around(full_schematic, x_idx.to_owned(), y_idx) {
            debug!("Found a symbol");
            part_number_flag = Some(cog_location);
            continue;
        }
    }

    if let Some(cog_location) = part_number_flag {
        let found_number = found_digits
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, dig)| dig * (10_u32.pow(idx as u32)))
            .sum::<u32>();

        gears.entry(cog_location).or_default().push(found_number);
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        let full_schematic: Vec<&[u8]> = lines.iter().map(|line| line.as_bytes()).collect();
        let mut gears: GearMap = HashMap::new();
        lines.iter().enumerate().for_each(|(x_idx, line)| {
            find_potential_gears(&x_idx, line, &full_schematic, &mut gears)
        });

        debug!("{:?}", gears);

        let sum: u32 = gears
            .iter()
            .map(|((_, _), gear)| {
                if gear.len() != 2 {
                    return 0;
                }

                gear[0] * gear[1]
            })
            .sum();
        sum.to_string()
    }
}

fn main() {
    pretty_env_logger::init();
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
