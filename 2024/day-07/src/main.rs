use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;
use ilog::IntLog;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Value = u64;
type Input = Vec<Equation>;
struct Equation {
    result: Value,
    operands: Vec<Value>,
}

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
        .map(|l| l.unwrap())
        .map(|l| {
            let (result_str, operands_str) = l.split_once(": ").unwrap();
            let result = result_str.parse::<Value>().unwrap();
            let operands = operands_str
                .split(" ")
                .map(|s| s.parse::<Value>().unwrap())
                .collect();

            Equation { result, operands }
        })
        .collect::<Input>())
}

const TWO: Value = 2;
const THREE: Value = 3;
const TEN: Value = 10;

fn part_one(input: &Input) -> Value {
    input
        .iter()
        .filter(|Equation { result, operands }| {
            let max = TWO.pow(operands.len() as u32);

            (0..max)
                .map(|i| {
                    let mut result = operands[0];
                    for (j, operand) in operands.iter().skip(1).enumerate() {
                        match i & (1 << j) == 0 {
                            true => result += operand,
                            false => result *= operand,
                        }
                    }

                    result
                })
                .any(|r| r == *result)
        })
        .map(|e| e.result)
        .sum()
}

fn part_two(input: &Input) -> Value {
    input
        .iter()
        .filter(|Equation { result, operands }| {
            let max = THREE.pow(operands.len() as u32);

            (0..max)
                .map(|i| {
                    let mut i = i;
                    let mut result = operands[0];

                    for operand in operands.iter().skip(1) {
                        match i % THREE {
                            0 => result += operand,
                            1 => result *= operand,
                            2 => result = result * (TEN.pow(operand.log10() as u32 + 1)) + operand,
                            _ => unreachable!(),
                        }

                        i /= THREE;
                    }

                    result
                })
                .any(|r| r == *result)
        })
        .map(|e| e.result)
        .sum()
}
