use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub enum Direction {
    Left,
    Right,
}

pub struct Input {
    pub directions: Vec<Direction>,
    pub elements: Vec<String>,
    pub left: HashMap<String, String>,
    pub right: HashMap<String, String>,
}

impl Input {
    pub fn parse(filename: &str) -> Input {
        let file = File::open(filename).expect("Failed to open input file");
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let directions: Vec<Direction> = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })
            .collect();
        let mut elements = Vec::<String>::new();
        let mut left = HashMap::<String, String>::new();
        let mut right = HashMap::<String, String>::new();

        lines.skip(1).map(|line| line.unwrap()).for_each(|line| {
            elements.push(String::from(&line[0..3]));
            left.insert(String::from(&line[0..3]), String::from(&line[7..10]));
            right.insert(String::from(&line[0..3]), String::from(&line[12..15]));
        });

        Input {
            directions,
            elements,
            left,
            right,
        }
    }
}
