use lazy_static::lazy_static;
use regex::Regex;

use crate::draw::Draw;

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"^Game (?<id>\d+): (?<draws>.*)").unwrap();
}

pub struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl Game {
    pub fn parse(line: &str) -> Game {
        let capture = GAME_REGEX.captures(line).expect("Failed to parse game");

        let id = capture.name("id").unwrap().as_str().parse::<u32>().unwrap();
        let draws = capture
            .name("draws")
            .unwrap()
            .as_str()
            .split("; ")
            .map(|draw| Draw::parse(draw))
            .collect();

        Game { id, draws }
    }

    pub fn min_bag(&self) -> Draw {
        self.draws.iter().fold(
            Draw {
                red: 0,
                green: 0,
                blue: 0,
            },
            |bag, draw| bag.ensure(draw),
        )
    }
}
