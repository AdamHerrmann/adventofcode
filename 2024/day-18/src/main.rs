use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
// const SIZE: usize = 7;
// const TIME: usize = 12;

const INPUT_FILE: &str = "./input.txt";
const SIZE: usize = 71;
const TIME: usize = 1024;

type Input = Vec<Position>;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

struct State {
    position: Position,
    score: Reverse<usize>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
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
        .map(|l| l.unwrap())
        .map(|l| {
            let mut nums = l.split(",").map(|n| n.parse::<usize>().unwrap());
            let result = Position {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
            };

            assert!(nums.next().is_none());
            result
        })
        .collect::<Input>())
}

fn part_one(input: &Input) -> usize {
    let start = Position { x: 0, y: 0 };
    let end = Position {
        x: SIZE - 1,
        y: SIZE - 1,
    };

    let corrupt = {
        let mut map = vec![vec![false; SIZE]; SIZE];
        input.iter().take(TIME).for_each(|p| {
            map[p.x][p.y] = true;
        });
        map
    };

    let mut visited = vec![vec![false; SIZE]; SIZE];
    let mut queue = BinaryHeap::<State>::new();

    queue.push(State {
        position: start,
        score: Reverse(0),
    });

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if current.position == end {
            return current.score.0;
        }

        let Position { x, y } = current.position;
        if visited[x][y] {
            continue;
        }

        visited[x][y] = true;
        let mut neighbors = vec![];

        if x > 0 {
            neighbors.push(Position { x: x - 1, y });
        }
        if x < SIZE - 1 {
            neighbors.push(Position { x: x + 1, y });
        }
        if y > 0 {
            neighbors.push(Position { x, y: y - 1 });
        }
        if y < SIZE - 1 {
            neighbors.push(Position { x, y: y + 1 });
        }

        for neighbor in neighbors {
            if visited[neighbor.x][neighbor.y] || corrupt[neighbor.x][neighbor.y] {
                continue;
            }
            queue.push(State {
                position: neighbor,
                score: Reverse(current.score.0 + 1),
            });
        }
    }

    unreachable!();
}

fn part_two(input: &Input) -> String {
    let mut low = 0;
    let mut high = input.len();

    while low < high - 1 {
        let mid = (low + high) / 2;

        if is_reachable(input, mid) {
            low = mid;
        } else {
            high = mid;
        }
    }

    format!("{},{}", input[low].x, input[low].y)
}

fn is_reachable(input: &Input, time: usize) -> bool {
    let start = Position { x: 0, y: 0 };
    let end = Position {
        x: SIZE - 1,
        y: SIZE - 1,
    };

    let corrupt = {
        let mut map = vec![vec![false; SIZE]; SIZE];
        input.iter().take(time).for_each(|p| {
            map[p.x][p.y] = true;
        });
        map
    };

    let mut visited = vec![vec![false; SIZE]; SIZE];
    let mut queue = BinaryHeap::<State>::new();

    queue.push(State {
        position: start,
        score: Reverse(0),
    });

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if current.position == end {
            return true;
        }

        let Position { x, y } = current.position;
        if visited[x][y] {
            continue;
        }

        visited[x][y] = true;
        let mut neighbors = vec![];

        if x > 0 {
            neighbors.push(Position { x: x - 1, y });
        }
        if x < SIZE - 1 {
            neighbors.push(Position { x: x + 1, y });
        }
        if y > 0 {
            neighbors.push(Position { x, y: y - 1 });
        }
        if y < SIZE - 1 {
            neighbors.push(Position { x, y: y + 1 });
        }

        for neighbor in neighbors {
            if visited[neighbor.x][neighbor.y] || corrupt[neighbor.x][neighbor.y] {
                continue;
            }
            queue.push(State {
                position: neighbor,
                score: Reverse(current.score.0 + 1),
            });
        }
    }

    return false;
}
