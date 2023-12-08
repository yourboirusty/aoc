use std::{collections::HashMap, iter::Cycle};

use base::main_day_fn;
use regex::Regex;

#[derive(Debug)]
struct CamelMap {
    steps: Vec<u8>,
    current: String,
    target: String,
    nodes: NodeMap,
}

type Node = (String, String);
type NodeMap = HashMap<String, Node>;

impl CamelMap {
    fn solve(&mut self) -> u64 {
        let step_cycle = self.steps.iter().cycle();
        let mut step_count = 0;
        for &step in step_cycle {
            step_count += 1;
            trace!("{} going {}", self.current, step);
            self.current = match step {
                0 => self.nodes.get(&self.current).unwrap().0.clone(),
                1 => self.nodes.get(&self.current).unwrap().1.clone(),
                _ => panic!("unknown step"),
            };
            if self.current == self.target {
                break;
            }
        }
        step_count
    }
}

impl<'a> FromIterator<&'a String> for CamelMap {
    fn from_iter<T: IntoIterator<Item = &'a String>>(iter: T) -> Self {
        let mut lines = iter.into_iter();
        let steps: Vec<u8> = lines
            .next()
            .unwrap()
            .bytes()
            .map(|c| match c as char {
                'R' => 1,
                'L' => 0,
                _ => panic!("Cant parse steps"),
            })
            .collect();

        lines.next();

        let mut nodes: NodeMap = HashMap::new();
        let re = Regex::new(r"(?<node_id>.{3}).*\((?<l_id>.{3}).*(?<r_id>.{3})\)").unwrap();

        for line in lines {
            let parsed_line = re.captures(line.as_str()).unwrap();
            let node_id = parsed_line["node_id"].to_owned();
            nodes.insert(
                node_id.to_owned(),
                (
                    parsed_line["l_id"].to_owned(),
                    parsed_line["r_id"].to_owned(),
                ),
            );
        }
        CamelMap {
            steps,
            current: "AAA".to_owned(),
            target: "ZZZ".to_owned(),
            nodes,
        }
    }
}

struct Part1;

impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let mut camel_map = CamelMap::from_iter(lines.iter());
        debug!("{:?}", camel_map);
        camel_map.solve().to_string()
    }
}

struct Part2;

impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        todo!();
    }
}

main_day_fn!(Part1, Part2);
