use crate::day::Day;
use crate::day::Solveable;

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let a = 600851475143;
        let mut prime: Vec<bool> = vec![true; (a as f64).sqrt() as usize + 1];
        prime[0] = false;
        prime[1] = false;
        for i in 2..prime.len() {
            if prime[i] {
                let mut j = i * i;
                while j < prime.len() {
                    prime[j] = false;
                    j += i;
                }
            }
        }
        (2..prime.len())
            .filter(|i| prime[*i as usize] && a % i == 0)
            .max()
            .unwrap()
            .to_string()
    }
}

pub fn get_day(pack_name: String) -> Day {
    Day::new(pack_name, 3, vec![Box::new(Part1)])
}
