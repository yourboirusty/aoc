use std::str::FromStr;

use crate::day::{Day, Solveable};

struct Game {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl Game {
    fn new(id: u32) -> Self {
        Game {
            id,
            max_blue: 0,
            max_red: 0,
            max_green: 0,
        }
    }

    fn fits(&self, other: &Game) -> bool {
        self.max_red >= other.max_red
            && self.max_green >= other.max_green
            && self.max_blue >= other.max_blue
    }
}

#[test]
fn test_fits() {
    let smaller = Game {
        id: 0,
        max_blue: 1,
        max_green: 1,
        max_red: 1,
    };
    let bigger = Game {
        id: 0,
        max_blue: 2,
        max_red: 2,
        max_green: 2,
    };

    assert!(bigger.fits(&smaller));
    assert!(!smaller.fits(&bigger));
}

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (metadata, sets_str) = s.split_once(':').unwrap();
        let id: u32 = metadata
            .split_once(' ')
            .unwrap()
            .1
            .parse()
            .map_err(|_| ParseGameError)?;

        let mut game = Game::new(id);
        sets_str.split(";").for_each(|set| {
            set.split(',').for_each(|ball_score| {
                let (score, ball) = ball_score.trim().split_once(" ").unwrap();
                let iscore: u32 = score.parse().map_err(|_| ParseGameError).unwrap();
                match ball {
                    "green" => game.max_green = std::cmp::max(game.max_green, iscore),
                    "blue" => game.max_blue = std::cmp::max(game.max_blue, iscore),
                    "red" => game.max_red = std::cmp::max(game.max_red, iscore),
                    &_ => (),
                }
            })
        });

        Ok(game)
    }
}

#[test]
fn test_parsing_works() {
    let test_str = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
    let game = Game::from_str(test_str).unwrap();
    assert!(game.id == 4);
    assert!(game.max_red == 14);
    assert!(game.max_blue == 15);
    assert!(game.max_green == 3);
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &[String]) -> String {
        let games = lines.iter().map(|line| Game::from_str(line).unwrap());
        let max_game = Game {
            id: 0,
            max_blue: 14,
            max_green: 13,
            max_red: 12,
        };
        let mut score = 0;
        for game in games {
            if max_game.fits(&game) {
                score += game.id
            }
        }
        score.to_string()
    }
}

struct Part2;
impl Solveable for Part2 {
    fn solve(&self, lines: &[String]) -> String {
        let games = lines.iter().map(|line| Game::from_str(line).unwrap());
        let score: u32 = games
            .map(|game| game.max_green * game.max_red * game.max_blue)
            .sum();
        score.to_string()
    }
}
get_day_fn!(Part1, Part2);
