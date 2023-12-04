use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

const RADIX: u32 = 10;

fn main() -> Result<()> {
    println!("Part 1: {}", part_one().unwrap());
    println!("Part 2: {}", part_two().unwrap());
    Ok(())
}

fn part_one() -> Result<u32> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut sum = 0;

    for line in reader.lines() {
        if let Ok(s) = line {
            let digits: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();

            let first = digits.first().unwrap().to_digit(RADIX).unwrap();
            let last = digits.last().unwrap().to_digit(RADIX).unwrap();
            sum = sum + first * 10 + last;
        }
    }

    Ok(sum)
}

fn part_two() -> Result<u32> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut sum = 0;

    for line in reader.lines() {
        if let Ok(s) = line {
            let first = first_digit(&s).unwrap();
            let last = last_digit(&s).unwrap();

            sum = sum + first * 10 + last;
        }
    }

    Ok(sum)
}

fn first_digit(s: &str) -> Option<u32> {
    match s.chars().next() {
        None => return None,
        Some(c) => {
            if c.is_numeric() {
                return c.to_digit(RADIX);
            }
        }
    }

    if s.starts_with("one") {
        return Some(1);
    }
    if s.starts_with("two") {
        return Some(2);
    }
    if s.starts_with("three") {
        return Some(3);
    }
    if s.starts_with("four") {
        return Some(4);
    }
    if s.starts_with("five") {
        return Some(5);
    }
    if s.starts_with("six") {
        return Some(6);
    }
    if s.starts_with("seven") {
        return Some(7);
    }
    if s.starts_with("eight") {
        return Some(8);
    }
    if s.starts_with("nine") {
        return Some(9);
    }

    return first_digit(&s[1..]);
}

fn last_digit(s: &str) -> Option<u32> {
    match s.chars().rev().next() {
        None => return None,
        Some(c) => {
            if c.is_numeric() {
                return c.to_digit(RADIX);
            }
        }
    }

    if s.ends_with("one") {
        return Some(1);
    }
    if s.ends_with("two") {
        return Some(2);
    }
    if s.ends_with("three") {
        return Some(3);
    }
    if s.ends_with("four") {
        return Some(4);
    }
    if s.ends_with("five") {
        return Some(5);
    }
    if s.ends_with("six") {
        return Some(6);
    }
    if s.ends_with("seven") {
        return Some(7);
    }
    if s.ends_with("eight") {
        return Some(8);
    }
    if s.ends_with("nine") {
        return Some(9);
    }

    return last_digit(&s[..s.len() - 1]);
}
