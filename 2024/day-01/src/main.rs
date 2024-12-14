use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

// const INPUT: &str = "./example.txt";
const INPUT: &str = "./input.txt";

type Input = Vec<(u32, u32)>;

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("Part 1: {}", part_one_answer);
    println!("Part 2: {}", part_two_answer);
    Ok(())
}

fn parse_input() -> Result<Input> {
    let reader = BufReader::new(File::open(INPUT)?);
    let line_re = Regex::new(r"(\d+)\s+(\d+)")?;

    reader
        .lines()
        .map(|line| {
            let line = line?;
            let captures = line_re.captures(&line).unwrap();

            Ok((captures[1].parse::<u32>()?, captures[2].parse::<u32>()?))
        })
        .collect()
}

fn part_one(input: &Input) -> u32 {
    let mut left = input.iter().map(|(left, _)| left).collect::<Vec<_>>();
    let mut right = input.iter().map(|(_, right)| right).collect::<Vec<_>>();

    left.sort();
    right.sort();

    left.iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn part_two(input: &Input) -> u32 {
    let mut right_count = HashMap::new();

    input.iter().map(|(_, right)| right).for_each(|right| {
        let count = right_count.get(right).unwrap_or(&0) + 1;
        right_count.insert(*right, count);
    });

    input
        .iter()
        .map(|(left, _)| left)
        .map(|left| left * right_count.get(left).unwrap_or(&0))
        .sum()
}
