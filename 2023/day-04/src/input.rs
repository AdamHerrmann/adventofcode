use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Input {
    pub cards: Vec<Card>,
}

pub struct Card {
    pub number: u32,
    pub winning_numbers: HashSet<u32>,
    pub your_numbers: HashSet<u32>,
}

pub fn parse(filename: &str) -> Input {
    let file = File::open(filename).expect("Failed to open input file");
    let cards = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_card(&l))
        .collect();

    Input { cards }
}

fn parse_card(line: &str) -> Card {
    let (card, numbers) = line.split_once(": ").unwrap();
    let number = card
        .split_whitespace()
        .collect::<Vec<&str>>()
        .get(1)
        .map(|id| id.parse::<u32>().unwrap())
        .unwrap();
    let (winning, yours) = numbers.split_once(" | ").unwrap();

    Card {
        number,
        winning_numbers: HashSet::from_iter(
            winning
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap()),
        ),
        your_numbers: HashSet::from_iter(
            yours.split_whitespace().map(|n| n.parse::<u32>().unwrap()),
        ),
    }
}
