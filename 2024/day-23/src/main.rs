use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

const INPUT_FILE: &str = "./example.txt";
// const INPUT_FILE: &str = "./input.txt";

type Input = Vec<Vec<char>>;

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("Part 1: {}", part_one_answer);
    println!("Part 2: {}", part_two_answer);

    Ok(())
}

fn parse_input() -> Result<Input> {
    let reader = BufReader::new(File::open(INPUT_FILE)?);

    Ok(reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Input>())
}

fn part_one(input: &Input) -> usize {
    input.len()
}

fn part_two(input: &Input) -> usize {
    input.len()
}
