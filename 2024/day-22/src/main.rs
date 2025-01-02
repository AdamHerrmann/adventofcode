use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;
use itertools::Itertools;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<u32>;

struct SecretNumberIter {
    secret: u32,
}

impl SecretNumberIter {
    fn new(initial_secret: u32) -> Self {
        Self {
            secret: initial_secret,
        }
    }
}

impl Iterator for SecretNumberIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.secret);

        self.secret = {
            let mut next = self.secret;
            next = ((next << 6) ^ next) & 0xffffff;
            next = ((next >> 5) ^ next) & 0xffffff;
            next = ((next << 11) ^ next) & 0xffffff;
            next
        };

        result
    }
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
        .map(|l| l.unwrap().parse().unwrap())
        .collect::<Input>())
}

fn part_one(input: &Input) -> u64 {
    input
        .iter()
        .map(|initial_secret| {
            SecretNumberIter::new(*initial_secret)
                .take(2001)
                .last()
                .unwrap() as u64
        })
        .sum()
}

fn part_two(input: &Input) -> usize {
    let mut total_bananas = [0usize; PRICES_SIZE];

    input
        .iter()
        .map(|initial_secret| {
            let mut bananas = [None; PRICES_SIZE];

            SecretNumberIter::new(*initial_secret)
                .take(2001)
                .map(|s| (s % 10) as i8)
                .tuple_windows()
                .map(|(a, b)| (b, b - a))
                .tuple_windows()
                .map(|((_, a), (_, b), (_, c), (v, d))| (changes_to_index(a, b, c, d), v))
                .for_each(|(index, price)| {
                    bananas[index].get_or_insert(price);
                });

            bananas
        })
        .for_each(|prices| {
            prices.iter().enumerate().for_each(|(index, bananas)| {
                total_bananas[index] += bananas.unwrap_or(0) as usize;
            });
        });

    *total_bananas.iter().max().unwrap()
}

const CHANGE_RADIX: usize = 19;
const CHANGE_OFFSET: i8 = 9;
const PRICES_SIZE: usize = CHANGE_RADIX * CHANGE_RADIX * CHANGE_RADIX * CHANGE_RADIX;

fn changes_to_index(a: i8, b: i8, c: i8, d: i8) -> usize {
    let mut result: usize = 0;

    result += (CHANGE_OFFSET + a) as usize;
    result *= CHANGE_RADIX;
    result += (CHANGE_OFFSET + b) as usize;
    result *= CHANGE_RADIX;
    result += (CHANGE_OFFSET + c) as usize;
    result *= CHANGE_RADIX;
    result += (CHANGE_OFFSET + d) as usize;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_iter() {
        let actual = SecretNumberIter::new(123).take(11).collect::<Vec<u32>>();
        let expected = vec![
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_one() {
        let examples = vec![
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ];

        for (initial_secret, expected) in examples {
            assert_eq!(
                SecretNumberIter::new(initial_secret)
                    .take(2001)
                    .last()
                    .unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_windows() {
        let actual = SecretNumberIter::new(123)
            .take(10)
            .map(|s| (s % 10) as i8)
            .tuple_windows()
            .map(|(a, b)| (b, b - a))
            .collect::<Vec<_>>();

        let expected = vec![
            (0, -3),
            (6, 6),
            (5, -1),
            (4, -1),
            (4, 0),
            (6, 2),
            (4, -2),
            (4, 0),
            (2, -2),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_changes() {
        let actual = SecretNumberIter::new(123)
            .take(10)
            .map(|s| (s % 10) as i8)
            .tuple_windows()
            .map(|(a, b)| (b, b - a))
            .tuple_windows()
            .map(|((_, a), (_, b), (_, c), (v, d))| (v, (a, b, c, d)))
            .collect::<Vec<_>>();

        let expected = vec![
            (4, (-3, 6, -1, -1)),
            (4, (6, -1, -1, 0)),
            (6, (-1, -1, 0, 2)),
            (4, (-1, 0, 2, -2)),
            (4, (0, 2, -2, 0)),
            (2, (2, -2, 0, -2)),
        ];

        assert_eq!(actual, expected);
    }
}
