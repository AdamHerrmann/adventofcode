use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Input {
    pub readings: Vec<Vec<i32>>,
}

pub fn parse(filename: &str) -> Input {
    let file = File::open(filename).expect("Failed to open input file");

    Input {
        readings: BufReader::new(file)
            .lines()
            .map(|line| {
                line.unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()
            })
            .collect(),
    }
}
