use std::{iter::zip, ops::Range};

use base::main_day_fn;

struct Part1;

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

type RaceSolution = u64;

impl Race {
    fn get_delta(&self) -> u64 {
        let delta = self.time.pow(2) - 4 * self.record;
        debug!("Delta {delta}");
        delta
    }

    fn get_delta_sqrt(&self) -> f64 {
        let delta_sqrt = (self.get_delta() as f64).sqrt();
        debug!("sqrt {delta_sqrt}");
        delta_sqrt
    }

    fn has_more_solutions(&self) -> bool {
        self.time != 2 * self.record
    }

    fn get_solution_count(&self) -> RaceSolution {
        debug!("{:?}", self);
        if !self.has_more_solutions() {
            debug!("No more solutions");
            return 0;
        }
        let upper_bound = ((self.time as f64 + self.get_delta_sqrt()) / 2_f64).ceil() - 1_f64;
        let lower_bound = ((self.time as f64 - self.get_delta_sqrt()) / 2_f64).floor() + 1_f64;
        debug!("Upper bound found: {upper_bound}");
        debug!("lower bound: {lower_bound}");
        (upper_bound - lower_bound) as u64 + 1
    }
}

impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let time_string = lines[0]
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap());
        let distance_string = lines[1]
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap());
        let mut race_solutions: Vec<RaceSolution> = Vec::new();

        for (time, record) in zip(time_string, distance_string) {
            let race = Race { time, record };
            race_solutions.push(race.get_solution_count());
        }
        debug!("solutions: {:?}", race_solutions);
        race_solutions
            .iter()
            .copied()
            .reduce(|a, b| a * b)
            .unwrap()
            .to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        let time_string = lines[0].split_once(':').unwrap().1.replace(' ', "");
        let distance_string = lines[1].split_once(':').unwrap().1.replace(' ', "");

        debug!("{distance_string}");
        let record = distance_string.parse().unwrap();
        let time = time_string.parse().unwrap();

        let race = Race { time, record };
        race.get_solution_count().to_string()
    }
}

main_day_fn!(Part1, Part2);
