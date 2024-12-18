use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<Machine>;
type Value = u64;

#[derive(Debug)]
struct Machine {
    pub button_a: Point,
    pub button_b: Point,
    pub prize: Point,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: Value,
    pub y: Value,
}

const A_COST: Value = 3;
const B_COST: Value = 1;
const CONVERSION_ERROR: Value = 10000000000000;

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("Part 1: {}", part_one_answer);
    println!("Part 2: {}", part_two_answer);

    Ok(())
}

enum ParseState {
    ButtonA,
    ButtonB(Point),
    Prize(Point, Point),
    Blank,
}

fn parse_input() -> Result<Input> {
    let reader = BufReader::new(File::open(INPUT_FILE)?);
    let mut state = ParseState::ButtonA;

    let button_a_regex = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_regex = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        match state {
            ParseState::ButtonA => {
                let (_, [x, y]) = button_a_regex.captures(&line).unwrap().extract();
                let x = x.parse::<Value>().unwrap();
                let y = y.parse::<Value>().unwrap();

                state = ParseState::ButtonB(Point { x, y });
            }
            ParseState::ButtonB(button_a) => {
                let (_, [x, y]) = button_b_regex.captures(&line).unwrap().extract();
                let x = x.parse::<Value>().unwrap();
                let y = y.parse::<Value>().unwrap();

                state = ParseState::Prize(button_a, Point { x, y });
            }
            ParseState::Prize(button_a, button_b) => {
                let (_, [x, y]) = prize_regex.captures(&line).unwrap().extract();
                let x = x.parse::<Value>().unwrap();
                let y = y.parse::<Value>().unwrap();

                result.push(Machine {
                    button_a,
                    button_b,
                    prize: Point { x, y },
                });
                state = ParseState::Blank;
            }
            ParseState::Blank => {
                state = ParseState::ButtonA;
            }
        }
    }

    Ok(result)
}

fn part_one(input: &Input) -> Value {
    // a_count * a_x + b_count * b_x = prize_x
    // a_count * a_y + b_count * b_y = prize_y
    //
    // a_count = (prize_x - b_count * b_x) / a_x
    // a_y * (prize_x - b_count * b_x) / a_x + b_count * b_y = prize_y
    // a_y * prize_x / a_x - a_y * b_count * b_x / a_x + b_count * b_y = prize_y
    // b_count * b_y - a_y * b_count * b_x / a_x = prize_y - a_y * prize_x / a_x
    // b_count * (b_y - a_y * b_x / a_x) = prize_y - a_y * prize_x / a_x
    // b_count = (prize_y - a_y * prize_x / a_x) / (b_y - a_y * b_x / a_x)

    // b_count = (ax * prize_y - a_y * prize_x ) / (a_x * b_y - a_y * b_x)

    input
        .iter()
        .filter_map(|machine| {
            let a_x = machine.button_a.x;
            let a_y = machine.button_a.y;
            let b_x = machine.button_b.x;
            let b_y = machine.button_b.y;
            let prize_x = machine.prize.x;
            let prize_y = machine.prize.y;

            if (a_x * prize_y < a_y * prize_x) ^ (a_x * b_y < a_y * b_x) {
                return None;
            }
            let b_count = (a_x * prize_y).abs_diff(a_y * prize_x) / (a_x * b_y).abs_diff(a_y * b_x);

            if prize_x < b_count * b_x {
                return None;
            }
            let a_count = (prize_x - b_count * b_x) / a_x;

            if a_count * a_x + b_count * b_x != prize_x || a_count * a_y + b_count * b_y != prize_y
            {
                return None;
            }

            Some((a_count, b_count))
        })
        .map(|(a_count, b_count)| A_COST * a_count + B_COST * b_count)
        .sum()
}

fn part_two(input: &Input) -> Value {
    input
        .iter()
        .filter_map(|machine| {
            let a_x = machine.button_a.x;
            let a_y = machine.button_a.y;
            let b_x = machine.button_b.x;
            let b_y = machine.button_b.y;
            let prize_x = machine.prize.x + CONVERSION_ERROR;
            let prize_y = machine.prize.y + CONVERSION_ERROR;

            if (a_x * prize_y < a_y * prize_x) ^ (a_x * b_y < a_y * b_x) {
                return None;
            }
            let b_count = (a_x * prize_y).abs_diff(a_y * prize_x) / (a_x * b_y).abs_diff(a_y * b_x);

            if prize_x < b_count * b_x {
                return None;
            }
            let a_count = (prize_x - b_count * b_x) / a_x;

            if a_count * a_x + b_count * b_x != prize_x || a_count * a_y + b_count * b_y != prize_y
            {
                return None;
            }

            Some((a_count, b_count))
        })
        .map(|(a_count, b_count)| A_COST * a_count + B_COST * b_count)
        .sum()
}
