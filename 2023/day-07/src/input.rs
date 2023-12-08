use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::hand::Hand;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]

pub struct Input {
    pub hands: Vec<Hand>,
}

impl Input {
    pub fn parse(filename: &str) -> Input {
        let file = File::open(filename).expect("Failed to open input file");
        let mut hands: Vec<Hand> = BufReader::new(file)
            .lines()
            .map(|line| Hand::parse(line.unwrap()))
            .collect();

        hands.sort();

        Input { hands }
    }
}
