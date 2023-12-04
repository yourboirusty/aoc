use core::fmt::Debug;

#[derive(Debug)]
pub struct Day {
    name: String,
    parts: Vec<Box<dyn Solveable + Sync>>,
}

impl Day {
    pub fn new(name: String, parts: Vec<Box<dyn Solveable + Sync>>) -> Day {
        Day { name, parts }
    }
}

pub trait Solveable {
    fn solve(&self, lines: &[String]) -> String;

    fn timed_solve(&self, lines: &[String]) -> (String, std::time::Duration) {
        let now = std::time::Instant::now();
        let solution = self.solve(lines);
        let elapsed = now.elapsed();

        (solution, elapsed)
    }
}

impl Debug for dyn Solveable + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Solveable")
    }
}

impl Day {
    pub fn solve(&self, lines_factory: &dyn Fn(&String) -> Vec<String>) {
        let lines = lines_factory(&self.name);
        self.parts.iter().enumerate().for_each(|(idx, part)| {
            let (ans, time) = part.timed_solve(&lines);
            println!("Part {}: {}, took {} s", idx + 1, ans, time.as_secs_f32());
        });
    }

    pub fn solve_standalone(&self, path: &String) {
        let lines: Vec<String> = std::fs::read_to_string(path.clone())
            .unwrap_or_else(|_| {
                warn!("{path} not found, defaulting to empty");
                String::new()
            })
            .lines()
            .map(|s| s.to_string())
            .collect();
        self.parts.iter().enumerate().for_each(|(idx, part)| {
            let (ans, time) = part.timed_solve(&lines);
            println!("Part {}: {}, took {} s", idx + 1, ans, time.as_secs_f32());
        });
    }
}
