use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<Vec<u8>>;

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
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Input>())
}

type Cache = Vec<Vec<HashSet<(usize, usize)>>>;

fn part_one(input: &Input) -> u32 {
    let height = input.len();
    let width = input[0].len();

    let mut cache: Cache = vec![vec![HashSet::default(); width]; height];

    iter_2d(height, width)
        .filter(|&(x, y)| input[x][y] == 9)
        .for_each(|(x, y)| {
            cache[x][y].insert((x, y));
        });

    (0..9).rev().for_each(|value| {
        iter_2d(height, width)
            .filter(|&(x, y)| input[x][y] == value)
            .for_each(|(x, y)| {
                let next = value + 1;
                let mut this_cache = HashSet::new();

                if x > 0 && input[x - 1][y] == next {
                    cache[x - 1][y].iter().for_each(|&(nx, ny)| {
                        this_cache.insert((nx, ny));
                    });
                }

                if x < height - 1 && input[x + 1][y] == next {
                    cache[x + 1][y].iter().for_each(|&(nx, ny)| {
                        this_cache.insert((nx, ny));
                    });
                }

                if y > 0 && input[x][y - 1] == next {
                    cache[x][y - 1].iter().for_each(|&(nx, ny)| {
                        this_cache.insert((nx, ny));
                    });
                }

                if y < width - 1 && input[x][y + 1] == next {
                    cache[x][y + 1].iter().for_each(|&(nx, ny)| {
                        this_cache.insert((nx, ny));
                    });
                }

                cache[x][y] = this_cache;
            });
    });

    iter_2d(height, width)
        .filter(|&(x, y)| input[x][y] == 0)
        .map(|(x, y)| cache[x][y].len() as u32)
        .sum()
}

fn iter_2d(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |row| (0..width).map(move |col| (row, col)))
}

fn part_two(input: &Input) -> u32 {
    let mut cache = vec![vec![Option::<u32>::None; input[0].len()]; input.len()];

    iter_2d(input.len(), input[0].len())
        .filter(|&(x, y)| input[x][y] == 0)
        .map(|(x, y)| paths_from(input, x, y, &mut cache))
        .sum::<u32>()
}

type PathsFromCache = Vec<Vec<Option<u32>>>;
fn paths_from(input: &Input, x: usize, y: usize, cache: &mut PathsFromCache) -> u32 {
    if let Some(result) = cache[x][y] {
        return result;
    }

    if input[x][y] == 9 {
        cache[x][y] = Some(1);
        return 1;
    }

    let next = input[x][y] + 1;
    let height = input.len();
    let width = input[0].len();

    let up = if x > 0 && input[x - 1][y] == next {
        paths_from(input, x - 1, y, cache)
    } else {
        0
    };

    let down = if x < height - 1 && input[x + 1][y] == next {
        paths_from(input, x + 1, y, cache)
    } else {
        0
    };

    let left = if y > 0 && input[x][y - 1] == next {
        paths_from(input, x, y - 1, cache)
    } else {
        0
    };

    let right = if y < width - 1 && input[x][y + 1] == next {
        paths_from(input, x, y + 1, cache)
    } else {
        0
    };

    let result = up + down + left + right;
    cache[x][y] = Some(result);

    result
}
