use ilog::IntLog;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Value = u64;
type Input = Vec<Value>;

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
        .flat_map(|l| {
            l.unwrap()
                .split(" ")
                .map(|s| s.parse::<Value>().unwrap())
                .collect::<Vec<Value>>()
        })
        .collect())
}

fn is_even_digits(v: Value) -> bool {
    let digits = v.log10() + 1;
    digits % 2 == 0
}

fn split_digits(v: Value) -> (Value, Value) {
    let digits = (v.log10() + 1) as u32;
    let splitter = (10 as Value).pow(digits / 2);

    (v / splitter, v % splitter)
}

type Cache = HashMap<Value, HashMap<u8, u64>>;

fn calculate_final_size(cache: &mut Cache, value: Value, steps: u8) -> u64 {
    if let Some(steps_cache) = cache.get(&value) {
        if let Some(&result) = steps_cache.get(&steps) {
            return result;
        }
    }

    if steps == 0 {
        return 1;
    }

    let result = match value {
        0 => calculate_final_size(cache, 1, steps - 1),
        v if is_even_digits(v) => {
            let (first, second) = split_digits(v);
            calculate_final_size(cache, first, steps - 1)
                + calculate_final_size(cache, second, steps - 1)
        }
        v => calculate_final_size(cache, v * 2024, steps - 1),
    };

    cache
        .entry(value)
        .or_insert_with(HashMap::new)
        .insert(steps, result);

    result
}

fn part_one(input: &Input) -> u64 {
    let mut cache = Cache::new();

    input
        .iter()
        .map(|&v| calculate_final_size(&mut cache, v, 25))
        .sum()
}

fn part_two(input: &Input) -> u64 {
    let mut cache = Cache::new();

    input
        .iter()
        .map(|&v| calculate_final_size(&mut cache, v, 75))
        .sum()
}
