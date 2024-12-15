use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<Vec<char>>;

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
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Input>())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Clone, Copy)]
struct Guard {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Guard {
    pub fn next(self, input: &Input) -> Option<Guard> {
        match self.direction {
            Direction::Up if self.row == 0 => None,
            Direction::Down if self.row == input.len() - 1 => None,
            Direction::Left if self.col == 0 => None,
            Direction::Right if self.col == input[0].len() - 1 => None,

            Direction::Up if input[self.row - 1][self.col] == '#' => Some(Guard {
                row: self.row,
                col: self.col,
                direction: Direction::Right,
            }),
            Direction::Down if input[self.row + 1][self.col] == '#' => Some(Guard {
                row: self.row,
                col: self.col,
                direction: Direction::Left,
            }),
            Direction::Left if input[self.row][self.col - 1] == '#' => Some(Guard {
                row: self.row,
                col: self.col,
                direction: Direction::Up,
            }),
            Direction::Right if input[self.row][self.col + 1] == '#' => Some(Guard {
                row: self.row,
                col: self.col,
                direction: Direction::Down,
            }),

            Direction::Up => Some(Guard {
                row: self.row - 1,
                col: self.col,
                direction: Direction::Up,
            }),
            Direction::Down => Some(Guard {
                row: self.row + 1,
                col: self.col,
                direction: Direction::Down,
            }),
            Direction::Left => Some(Guard {
                row: self.row,
                col: self.col - 1,
                direction: Direction::Left,
            }),
            Direction::Right => Some(Guard {
                row: self.row,
                col: self.col + 1,
                direction: Direction::Right,
            }),
        }
    }
}

fn get_start(input: &Input) -> Guard {
    let start = (0..input.len())
        .into_iter()
        .flat_map(|row| (0..input[row].len()).into_iter().map(move |col| (row, col)))
        .find(|(row, col)| input[*row][*col] == '^')
        .unwrap();

    Guard {
        row: start.0,
        col: start.1,
        direction: Direction::Up,
    }
}

fn part_one(input: &Input) -> usize {
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut guard = Some(get_start(input));

    while let Some(current) = guard {
        visited[current.row][current.col] = true;
        guard = current.next(input);
    }

    visited.iter().flatten().filter(|&&v| v).count()
}

#[derive(Clone, Default)]
struct Visited {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Visited {
    pub fn visit(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
            Direction::Left => self.left = true,
            Direction::Right => self.right = true,
        };
    }

    pub fn is_visited_from(&self, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

fn part_two(input: &Input) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut visited = vec![vec![Visited::default(); input[0].len()]; input.len()];
    let guard_start = get_start(&input);
    let mut guard = Some(guard_start);

    while let Some(current) = guard {
        visited[current.row][current.col].visit(current.direction);
        guard = current.next(&input);
    }

    let mut unique = HashSet::new();
    visited
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .flat_map(move |(col_index, visited)| {
                    ALL_DIRECTIONS
                        .iter()
                        .filter(|&&d| visited.is_visited_from(d))
                        .map(move |&d| (row_index, col_index, d))
                })
        })
        .filter(|&(row, col, direction)| match direction {
            Direction::Up if row == 0 => false,
            Direction::Down if row == height - 1 => false,
            Direction::Left if col == 0 => false,
            Direction::Right if col == width - 1 => false,
            _ => true,
        })
        .map(|(row, col, direction)| match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        })
        .filter(|&(row, col)| !(row == guard_start.row && col == guard_start.col))
        .filter(|position| {
            if unique.contains(position) {
                false
            } else {
                unique.insert(*position);
                true
            }
        })
        .map(|(row, col)| {
            let mut input = input.clone();
            input[row][col] = '#';
            input
        })
        .filter(|input| would_loop(input))
        .count()
}

fn would_loop(input: &Input) -> bool {
    let mut visited = vec![vec![Visited::default(); input[0].len()]; input.len()];
    let mut guard = Some(get_start(&input));

    while let Some(current) = guard {
        if visited[current.row][current.col].is_visited_from(current.direction) {
            return true;
        }

        visited[current.row][current.col].visit(current.direction);
        guard = current.next(&input);
    }

    false
}
