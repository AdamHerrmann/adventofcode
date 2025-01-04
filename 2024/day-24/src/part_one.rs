use std::collections::HashMap;
use std::fmt::Debug;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example_1.txt";
// const INPUT_FILE: &str = "./example_2.txt";
const INPUT_FILE: &str = "./input.txt";

struct Input {
    initial_values: Vec<WireValue>,
    gates: Vec<Gate>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Wire([char; 3]);

#[derive(Clone, Copy, Debug)]
struct WireValue {
    wire: Wire,
    value: bool,
}

#[derive(Clone, Copy)]
struct Gate {
    left: Wire,
    right: Wire,
    out: Wire,
    operation: Operation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_one_answer = part_one(&input);

    println!("Part 1: {}", part_one_answer);
    Ok(())
}

fn parse_input() -> Result<Input> {
    let reader = BufReader::new(File::open(INPUT_FILE)?);
    let mut lines = reader.lines();

    let mut initial_values = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }

        let wire = parse_wire(&line);
        let value = match line.chars().nth(5).unwrap() {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid value"),
        };

        initial_values.push(WireValue { wire, value });
    }

    let gates = lines
        .map(|l| l.unwrap())
        .map(|l| {
            let mut parts = l.split(" ");
            let left = parse_wire(parts.next().unwrap());
            let operation = match parts.next().unwrap() {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("Invalid operation"),
            };
            let right = parse_wire(parts.next().unwrap());
            assert_eq!(parts.next().unwrap(), "->");
            let out = parse_wire(parts.next().unwrap());
            assert!(parts.next().is_none());

            Gate {
                left,
                right,
                out,
                operation,
            }
        })
        .collect();

    Ok(Input {
        initial_values,
        gates,
    })
}

fn parse_wire(input: &str) -> Wire {
    assert!(input.len() >= 3);
    Wire([
        input.chars().nth(0).unwrap(),
        input.chars().nth(1).unwrap(),
        input.chars().nth(2).unwrap(),
    ])
}

fn part_one(input: &Input) -> u64 {
    let mut wire_values = HashMap::new();

    for value in &input.initial_values {
        wire_values.insert(value.wire, value.value);
    }

    let mut gates = input.gates.clone();

    while !gates.is_empty() {
        let (ready, not_ready) = gates.iter().partition::<Vec<Gate>, _>(|gate| {
            wire_values.contains_key(&gate.left) && wire_values.contains_key(&gate.right)
        });

        for gate in ready {
            let left = wire_values[&gate.left];
            let right = wire_values[&gate.right];
            let out = match gate.operation {
                Operation::And => left & right,
                Operation::Or => left | right,
                Operation::Xor => left ^ right,
            };

            wire_values.insert(gate.out, out);
        }

        gates = not_ready;
    }

    let mut result = 0;

    for (wire, value) in wire_values {
        let Wire([a, b, c]) = wire;

        if a == 'z' {
            let index = digit_to_number(b) * 10 + digit_to_number(c);
            result |= (value as u64) << index;
        }
    }

    result
}

fn digit_to_number(d: char) -> usize {
    d as usize - '0' as usize
}
