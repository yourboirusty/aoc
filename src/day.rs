use core::fmt::Debug;

#[derive(Debug)]
pub struct Day {
    pack_name: String,
    day: u8,
    part1: Box<dyn Solveable + Sync>,
    part2: Box<dyn Solveable + Sync>,
}

impl Day {
    pub fn new(pack_name: String, day: u8, part1: Box<dyn Solveable + Sync>, part2: Box<dyn Solveable + Sync>) -> Day {
        Day { pack_name, day, part1, part2 }
    }
}

pub trait Solveable {
    fn solve(&self, lines: &Vec<String>) -> String;
}

impl Debug for dyn Solveable + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Solveable")
    }
}

fn timed_solve(solve: &Box<dyn Solveable + Sync>, lines: &Vec<String>) -> (String, std::time::Duration) {
    let now = std::time::Instant::now();
    let solution = solve.solve(lines);
    let elapsed = now.elapsed();

    (solution, elapsed)
}


impl Solveable for Day {
    fn solve(&self, lines: &Vec<String>) -> String {
        let (p1_ans, p1_time) = timed_solve(&self.part1, &lines);
        println!("Part 1: {}, took {} s", p1_ans, p1_time.as_secs_f32());
        let (p2_ans, p2_time) = timed_solve(&self.part2, &lines);
        println!("Part 2: {}, took {} s", p2_ans, p2_time.as_secs_f32());

        String::new()
    }
}

pub fn read_lines_for(day: Day) -> Vec<String> {
    let path = format!("./input/{}/day{}.txt", day.pack_name, day.day);
    std::fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}
