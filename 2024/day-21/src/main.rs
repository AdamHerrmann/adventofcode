use smallvec::{smallvec, SmallVec};
use std::cmp::min;
use std::collections::HashMap;
use std::io::prelude::*;
use std::iter::{once, repeat};
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<Code>;

#[derive(Debug)]
struct Code {
    value: usize,
    digits: SmallVec<[char; 4]>,
}

type Move = SmallVec<[char; 6]>;
type MoveLengthCache = HashMap<(Move, usize), usize>;

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
            let mut chars = l.chars();
            let digits = smallvec![
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ];
            assert!(chars.next().is_none());

            Code {
                value: l[0..3].parse().unwrap(),
                digits,
            }
        })
        .collect())
}

fn numeric_keypad_position(digit: char) -> (u32, u32) {
    match digit {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!("Invalid digit: '{}'", digit),
    }
}

fn directional_keypad_position(direction: char) -> (u32, u32) {
    match direction {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!("Invalid direction: '{}'", direction),
    }
}

fn to_directions(
    from: &(u32, u32),
    to: &(u32, u32),
) -> (impl Iterator<Item = char>, impl Iterator<Item = char>) {
    let vertical_diff = from.0.abs_diff(to.0) as usize;
    let horizontal_diff = from.1.abs_diff(to.1) as usize;

    let vertical_char = if from.0 < to.0 { 'v' } else { '^' };
    let horizontal_char = if from.1 < to.1 { '>' } else { '<' };

    (
        repeat(vertical_char).take(vertical_diff),
        repeat(horizontal_char).take(horizontal_diff),
    )
}

fn press_number(from: (u32, u32), to: (u32, u32)) -> (Option<Move>, Option<Move>) {
    if from == to {
        return (Some(smallvec!['A']), None);
    }

    let mut result = (None, None);

    // conditions where moving vertically first is not allowed
    //   - from and to have the same .0 values
    //   - from is in the first column and to is in the last row
    if from.0 != to.0 && !(from.1 == 0 && to.0 == 3) {
        let (vertical, horizontal) = to_directions(&from, &to);

        result.0 = Some(vertical.chain(horizontal).chain(once('A')).collect());
    }

    // conditions where moving horizontally first is not allowed
    //  - from and to have the same .1 values
    //  - from is in the last row and to is in the first column
    if from.1 != to.1 && !(from.0 == 3 && to.1 == 0) {
        let (vertical, horizontal) = to_directions(&from, &to);

        result.1 = Some(horizontal.chain(vertical).chain(once('A')).collect());
    }

    assert!(result.0.is_some() || result.1.is_some());
    result
}

fn press_direction(from: (u32, u32), to: (u32, u32)) -> (Option<Move>, Option<Move>) {
    if from == to {
        return (Some(smallvec!['A']), None);
    }

    let mut result = (None, None);

    // conditions where moving vertically first is not allowed
    //  - if we're not moving vertically (then we'll be handled by the horizontal first below)
    //  - if moving vertically would take us to (0, 0)
    if from.0 != to.0 && from.1 != 0 {
        let (vertical, horizontal) = to_directions(&from, &to);
        result.0 = Some(vertical.chain(horizontal).chain(once('A')).collect());
    }

    // conditions where moving horizontally first is not allowed
    //   - if we're not moving horizontally (then we'll be handled by the vertical first above)
    //   - if moving horizontally would take us to (0, 0)
    if from.1 != to.1 && (to.1 != 0 || from.0 != 0) {
        let (vertical, horizontal) = to_directions(&from, &to);
        result.1 = Some(horizontal.chain(vertical).chain(once('A')).collect());
    }

    if result.0.is_none() && result.1.is_none() {
        println!("from: {:?}, to: {:?}", from, to);
    }

    assert!(result.0.is_some() || result.1.is_some());
    result
}

fn code_length(code: &Code, levels: usize, cache: &mut MoveLengthCache) -> usize {
    let mut position = (3, 2);

    code.digits
        .iter()
        .map(move |digit| {
            let next_position = numeric_keypad_position(*digit);
            let (left, right) = press_number(position, next_position);

            let left_length = left
                .map(|m| move_length(&m, levels, cache))
                .unwrap_or(usize::MAX);
            let right_length = right
                .map(|m| move_length(&m, levels, cache))
                .unwrap_or(usize::MAX);

            position = next_position;
            min(left_length, right_length)
        })
        .sum()
}

fn move_length(m: &Move, level: usize, cache: &mut MoveLengthCache) -> usize {
    if level == 0 {
        return m.len();
    }

    let key = (m.clone(), level);
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let mut position = (0, 2);
    let result = m
        .iter()
        .map(|direction| {
            let next_position = directional_keypad_position(*direction);
            let (left, right) = press_direction(position, next_position);

            let left_length = left
                .map(|m| move_length(&m, level - 1, cache))
                .unwrap_or(usize::MAX);
            let right_length = right
                .map(|m| move_length(&m, level - 1, cache))
                .unwrap_or(usize::MAX);

            position = next_position;
            min(left_length, right_length)
        })
        .sum();

    cache.insert(key, result);
    result
}

fn part_one(input: &Input) -> usize {
    let mut cache = MoveLengthCache::new();

    input
        .iter()
        .map(|code| code.value * code_length(code, 2, &mut cache))
        .sum()
}

fn part_two(input: &Input) -> usize {
    let mut cache = MoveLengthCache::new();

    input
        .iter()
        .map(|code| code.value * code_length(code, 25, &mut cache))
        .sum()
}
