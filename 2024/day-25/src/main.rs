use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

#[derive(Debug)]
struct Input {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

type Lock = [u8; 5];
type Key = [u8; 5];

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_one_answer = part_one(&input);

    println!("Part 1: {}", part_one_answer);
    Ok(())
}

fn parse_input() -> Result<Input> {
    let content = {
        let mut reader = BufReader::new(File::open(INPUT_FILE)?);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        content
    };

    let mut locks = vec![];
    let mut keys = vec![];

    for block in content.split("\n\n") {
        let counts = block
            .split("\n")
            .skip(1)
            .take(5)
            .map(|line| {
                let mut counts = line.chars().map(|c| match c {
                    '#' => 1u8,
                    '.' => 0u8,
                    _ => unreachable!("Invalid character in block"),
                });

                let result = [
                    counts.next().unwrap(),
                    counts.next().unwrap(),
                    counts.next().unwrap(),
                    counts.next().unwrap(),
                    counts.next().unwrap(),
                ];
                assert!(counts.next().is_none());
                result
            })
            .reduce(|acc, counts| {
                [
                    acc[0] + counts[0],
                    acc[1] + counts[1],
                    acc[2] + counts[2],
                    acc[3] + counts[3],
                    acc[4] + counts[4],
                ]
            })
            .unwrap();

        if block.starts_with('#') {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    }

    Ok(Input { locks, keys })
}

fn part_one(input: &Input) -> usize {
    input
        .locks
        .iter()
        .map(|lock| {
            input
                .keys
                .iter()
                .filter(|key| !lock.iter().zip(key.iter()).any(|(l, k)| l + k > 5))
                .count()
        })
        .sum()
}
