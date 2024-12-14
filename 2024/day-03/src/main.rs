use std::fs;

use anyhow::Result;
use regex::Regex;

// const INPUT: &str = "./example_1.txt";
// const INPUT: &str = "./example_2.txt";
const INPUT: &str = "./input.txt";

type Input = String;

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("Part 1: {}", part_one_answer);
    println!("Part 2: {}", part_two_answer);
    Ok(())
}

fn parse_input() -> Result<Input> {
    Ok(fs::read_to_string(INPUT)?)
}

fn part_one(input: &Input) -> u32 {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .map(|(a, b)| a * b)
        .sum()
}

#[derive(Debug)]
enum Expr {
    Mul(u32, u32),
    Do,
    Dont,
}

fn part_two(input: &Input) -> u32 {
    let mut enabled = true;

    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let full = c.get(0).unwrap().as_str();

            if full.starts_with("mul") {
                let a = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let b = c.get(2).unwrap().as_str().parse::<u32>().unwrap();

                Expr::Mul(a, b)
            } else if full.starts_with("don't") {
                Expr::Dont
            } else if full.starts_with("do") {
                Expr::Do
            } else {
                panic!("unexpected");
            }
        })
        .filter_map(|op| match op {
            Expr::Mul(a, b) => {
                if enabled {
                    Some((a, b))
                } else {
                    None
                }
            }
            Expr::Do => {
                enabled = true;
                None
            }
            Expr::Dont => {
                enabled = false;
                None
            }
        })
        .map(|(a, b)| a * b)
        .sum()
}
