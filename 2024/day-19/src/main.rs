use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

struct Input {
    towels: Vec<Towel>,
    patterns: Vec<Pattern>,
}

type Towel = Vec<Color>;
type Pattern = Vec<Color>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Cache = HashMap<String, usize>;

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
    let mut lines = reader.lines();

    let towels = lines
        .next()
        .unwrap()?
        .split(", ")
        .map(|s| parse_pattern(&s))
        .collect();

    assert_eq!(lines.next().unwrap().unwrap(), "");

    let patterns = lines
        .map(|line| line.unwrap())
        .map(|s| parse_pattern(&s))
        .collect();

    Ok(Input { towels, patterns })
}

fn parse_pattern(pattern: &str) -> Pattern {
    pattern.chars().map(parse_color).collect()
}

fn format_pattern(pattern: &[Color]) -> String {
    pattern.iter().map(format_color).collect()
}

fn parse_color(c: char) -> Color {
    match c {
        'w' => Color::White,
        'u' => Color::Blue,
        'b' => Color::Black,
        'r' => Color::Red,
        'g' => Color::Green,
        _ => panic!("Invalid color"),
    }
}

fn format_color(color: &Color) -> char {
    match color {
        Color::White => 'w',
        Color::Blue => 'u',
        Color::Black => 'b',
        Color::Red => 'r',
        Color::Green => 'g',
    }
}

fn part_one(input: &Input) -> usize {
    input
        .patterns
        .iter()
        .filter(|pattern| can_make_pattern(&input.towels, pattern))
        .count()
}

fn can_make_pattern(towels: &Vec<Towel>, pattern: &[Color]) -> bool {
    if pattern.is_empty() {
        return true;
    }

    for towel in towels {
        if towel_matches_pattern(towel, pattern)
            && can_make_pattern(towels, &pattern[towel.len()..])
        {
            return true;
        }
    }

    return false;
}

fn towel_matches_pattern(towel: &Towel, pattern: &[Color]) -> bool {
    towel.len() <= pattern.len()
        && towel
            .iter()
            .zip(pattern)
            .all(|(towel_color, pattern_color)| towel_color == pattern_color)
}

fn part_two(input: &Input) -> usize {
    let mut cache = Cache::new();

    input
        .patterns
        .iter()
        .filter(|pattern| can_make_pattern(&input.towels, pattern))
        .map(|pattern| count_the_ways(&mut cache, &input, pattern))
        .sum()
}

fn count_the_ways(cache: &mut Cache, input: &Input, pattern: &[Color]) -> usize {
    let mut ways = 0;

    if pattern.is_empty() {
        return 1;
    }

    let key = format_pattern(pattern);
    if let Some(&cached_ways) = cache.get(&key) {
        return cached_ways;
    }

    for towel in &input.towels {
        if towel_matches_pattern(towel, pattern) {
            ways += count_the_ways(cache, input, &pattern[towel.len()..]);
        }
    }

    cache.insert(key, ways);
    ways
}
