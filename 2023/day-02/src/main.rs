use std::io::prelude::*;
use std::{fs, io};

use draw::Draw;
use game::Game;

mod draw;
mod game;

fn main() {
    let file = fs::File::open("input.txt").expect("Failed to open input file");
    let games = io::BufReader::new(file)
        .lines()
        .map(|line| Game::parse(line.unwrap().as_str()))
        .collect();

    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));
}

fn part_one(games: &Vec<Game>) -> u32 {
    let bag = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter(|game| game.draws.iter().all(|draw| bag.allows(draw)))
        .fold(0, |sum, game| sum + game.id)
}

fn part_two(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| game.min_bag())
        .fold(0, |sum, bag| sum + bag.power())
}
