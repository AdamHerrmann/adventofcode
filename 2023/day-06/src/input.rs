use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::race::Race;

pub struct InputOne {
    pub races: Vec<Race>,
}

pub fn parse_input_one(filename: &str) -> InputOne {
    let file = File::open(filename).expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let time = &lines.next().unwrap().unwrap()[10..];
    let time = time
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap());

    let distance = &lines.next().unwrap().unwrap()[10..];
    let distance = distance
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap());

    InputOne {
        races: time
            .zip(distance)
            .map(|(time, distance)| Race { time, distance })
            .collect(),
    }
}

pub fn parse_input_two(filename: &str) -> Race {
    let file = File::open(filename).expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let time = parse_digits(lines.next().unwrap().unwrap());
    let distance = parse_digits(lines.next().unwrap().unwrap());

    Race { time, distance }
}

fn parse_digits(input: String) -> u64 {
    input
        .chars()
        .filter(|f| f.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}
