use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<(usize, usize)>;

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

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut visited: Vec<Vec<bool>> = reader
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '#' => true,
                    '.' => false,
                    'S' => {
                        start = (x, y);
                        false
                    }
                    'E' => {
                        end = (x, y);
                        false
                    }
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();

    let mut path = vec![start];
    let mut current = start;
    visited[current.0][current.1] = true;

    while current != end {
        current = find_next_step(&visited, current);

        path.push(current);
        visited[current.0][current.1] = true;
    }

    Ok(path)
}

fn find_next_step(visited: &Vec<Vec<bool>>, (x, y): (usize, usize)) -> (usize, usize) {
    if x > 0 && !visited[x - 1][y] {
        return (x - 1, y);
    }

    if y > 0 && !visited[x][y - 1] {
        return (x, y - 1);
    }

    if x < visited.len() - 1 && !visited[x + 1][y] {
        return (x + 1, y);
    }

    if y < visited[0].len() - 1 && !visited[x][y + 1] {
        return (x, y + 1);
    }

    unreachable!("Failed to find next step in path");
}

fn dist(from: (usize, usize), to: (usize, usize)) -> usize {
    let (x1, y1) = from;
    let (x2, y2) = to;

    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn part_one(input: &Input) -> usize {
    const CHEAT_LENGTH: usize = 2;
    const CHEAT_THRESHOLD: usize = 100;
    let mut count = 0;

    for start in 0..input.len() {
        for end in start + 1..input.len() {
            let cheat_length = dist(input[start], input[end]);
            let cheat_score = end - start - cheat_length;

            if cheat_length <= CHEAT_LENGTH && cheat_score >= CHEAT_THRESHOLD {
                count += 1;
            }
        }
    }

    count
}

fn part_two(input: &Input) -> usize {
    const CHEAT_LENGTH: usize = 20;
    const CHEAT_THRESHOLD: usize = 100;
    let mut count = 0;

    for start in 0..input.len() {
        for end in start + 1..input.len() {
            let cheat_length = dist(input[start], input[end]);
            let cheat_score = end - start - cheat_length;

            if cheat_length <= CHEAT_LENGTH && cheat_score >= CHEAT_THRESHOLD {
                count += 1;
            }
        }
    }

    count
}
