use core::fmt::Debug;

#[derive(Debug)]
pub struct Day {
    pack_name: String,
    day: u8,
    parts: Vec<Box<dyn Solveable + Sync>>,
}

impl Day {
    pub fn new(pack_name: String, day: u8, parts: Vec<Box<dyn Solveable + Sync>>) -> Day {
        Day {
            pack_name,
            day,
            parts,
        }
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

fn timed_solve(
    solve: &Box<dyn Solveable + Sync>,
    lines: &Vec<String>,
) -> (String, std::time::Duration) {
    let now = std::time::Instant::now();
    let solution = solve.solve(lines);
    let elapsed = now.elapsed();

    (solution, elapsed)
}

impl Day {
    pub fn solve(&self) {
        let lines = self.read_lines();
        self.parts.iter().enumerate().for_each(|(idx, part)| {
            let (ans, time) = timed_solve(part, &lines);
            println!("Part {}: {}, took {} s", idx + 1, ans, time.as_secs_f32());
        });
    }
    fn read_lines(&self) -> Vec<String> {
        let path = format!("./input/{}/day{}.txt", &self.pack_name, &self.day);
        std::fs::read_to_string(path)
            .unwrap_or_else(|_| {
                println!("File not found, defaulting to empty");
                String::new()
            })
            .lines()
            .map(|s| s.to_string())
            .collect()
    }
}

