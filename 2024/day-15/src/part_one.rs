use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example_1.txt";
// const INPUT_FILE: &str = "./example_2.txt";
const INPUT_FILE: &str = "./input.txt";

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_one_answer = part_one(&input);

    println!("Part 1: {}", part_one_answer);
    Ok(())
}

struct Input {
    pos: Pos,
    map: Map,
    moves: Vec<Move>,
}

#[derive(Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
type Map = Vec<Vec<Object>>;

#[derive(Clone, Copy)]
enum Object {
    Empty,
    Wall,
    Box,
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

enum ParseState {
    Map,
    Moves,
}

fn parse_input() -> Result<Input> {
    let reader = BufReader::new(File::open(INPUT_FILE)?);
    let mut state = ParseState::Map;
    let mut map = vec![];
    let mut moves = vec![];
    let mut pos = Pos { x: 0, y: 0 };

    for (x, line) in reader.lines().enumerate() {
        let line = line?;

        if line.is_empty() {
            state = ParseState::Moves;
            continue;
        }

        match state {
            ParseState::Map => {
                map.push(
                    line.chars()
                        .enumerate()
                        .map(|(y, c)| match c {
                            '.' => Object::Empty,
                            '#' => Object::Wall,
                            'O' => Object::Box,
                            '@' => {
                                pos.x = x;
                                pos.y = y;
                                Object::Empty
                            }
                            _ => panic!("Invalid character: {}", c),
                        })
                        .collect(),
                );
            }
            ParseState::Moves => line
                .chars()
                .map(|c| match c {
                    '<' => Move::Left,
                    '>' => Move::Right,
                    '^' => Move::Up,
                    'v' => Move::Down,
                    _ => panic!("Invalid character: {}", c),
                })
                .for_each(|m| moves.push(m)),
        }
    }

    if pos.x == 0 || pos.y == 0 {
        panic!("Invalid starting position");
    }

    Ok(Input { pos, map, moves })
}

fn part_one(input: &Input) -> usize {
    let mut pos = input.pos;
    let mut map = input.map.clone();

    input.moves.iter().for_each(|m| {
        if apply_move(&mut map, &pos, &m) {
            match m {
                Move::Up => pos.x -= 1,
                Move::Down => pos.x += 1,
                Move::Left => pos.y -= 1,
                Move::Right => pos.y += 1,
            }
        }
    });

    iter_2d(map.len(), map[0].len())
        .filter(|&(x, y)| match map[x][y] {
            Object::Box => true,
            _ => false,
        })
        .map(|(x, y)| 100 * x + y)
        .sum()
}

fn apply_move(map: &mut Map, pos: &Pos, m: &Move) -> bool {
    let next = match m {
        Move::Up => Pos {
            x: pos.x - 1,
            y: pos.y,
        },
        Move::Down => Pos {
            x: pos.x + 1,
            y: pos.y,
        },
        Move::Left => Pos {
            x: pos.x,
            y: pos.y - 1,
        },
        Move::Right => Pos {
            x: pos.x,
            y: pos.y + 1,
        },
    };

    match map[next.x][next.y] {
        Object::Wall => false,
        Object::Empty => {
            map[next.x][next.y] = map[pos.x][pos.y];
            map[pos.x][pos.y] = Object::Empty;
            true
        }
        Object::Box => {
            if apply_move(map, &next, m) {
                map[next.x][next.y] = map[pos.x][pos.y];
                map[pos.x][pos.y] = Object::Empty;
                true
            } else {
                false
            }
        }
    }
}

fn iter_2d(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |row| (0..width).map(move |col| (row, col)))
}
