use anyhow::Result;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

// const INPUT: &str = "./example.txt";
const INPUT: &str = "./input.txt";

type Input = Vec<Vec<u32>>;

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

    reader
        .lines()
        .map(|line| {
            let line = line?;
            let levels = line
                .split(" ")
                .map(|level| level.parse::<u32>().unwrap())
                .collect();

            Ok(levels)
        })
        .collect()
}

fn to_tuple(window: &[u32]) -> (u32, u32) {
    let [a, b] = window else { unreachable!() };
    (*a, *b)
}

fn is_increasing_safely(a: u32, b: u32) -> bool {
    a < b && a.abs_diff(b) <= 3
}

fn is_decreasing_safely(a: u32, b: u32) -> bool {
    a > b && a.abs_diff(b) <= 3
}

fn pairs<'a>(levels: &'a Vec<u32>) -> impl Iterator<Item = (u32, u32)> + 'a {
    levels.windows(2).map(to_tuple)
}

fn part_one(input: &Input) -> u32 {
    input
        .iter()
        .filter(|levels| {
            pairs(levels).all(|(a, b)| is_increasing_safely(a, b))
                || pairs(levels).all(|(a, b)| is_decreasing_safely(a, b))
        })
        .count() as u32
}

fn pairs_skip<'a>(levels: &'a Vec<u32>, skip: usize) -> impl Iterator<Item = (u32, u32)> + 'a {
    let mut previous = None;

    levels.iter().enumerate().filter_map(move |(i, &v)| {
        if i == skip {
            return None;
        }

        match previous {
            Some(p) => {
                previous = Some(v);
                Some((p, v))
            }
            None => {
                previous = Some(v);
                None
            }
        }
    })
}

fn part_two(input: &Input) -> u32 {
    input
        .iter()
        .filter(|levels| {
            let not_inc = levels
                .windows(2)
                .map(to_tuple)
                .enumerate()
                .find(|&(_, (a, b))| !is_increasing_safely(a, b));

            if not_inc.is_none() {
                return true;
            }

            let not_dec = levels
                .windows(2)
                .map(to_tuple)
                .enumerate()
                .find(|&(_, (a, b))| !is_decreasing_safely(a, b));

            if not_dec.is_none() {
                return true;
            }

            let not_inc = not_inc.unwrap();
            let not_dec = not_dec.unwrap();

            // trading extra work for easy stack allocation.
            let try_skip = [not_inc.0, not_inc.0 + 1, not_dec.0, not_dec.0 + 1];

            try_skip.iter().any(|&skip| {
                pairs_skip(levels, skip).all(|(a, b)| is_increasing_safely(a, b))
                    || pairs_skip(levels, skip).all(|(a, b)| is_decreasing_safely(a, b))
            })
        })
        .count() as u32
}
