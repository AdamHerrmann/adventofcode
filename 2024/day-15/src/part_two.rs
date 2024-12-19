use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example_1.txt";
// const INPUT_FILE: &str = "./example_2.txt";
// const INPUT_FILE: &str = "./example_3.txt";
const INPUT_FILE: &str = "./input.txt";

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_two_answer = part_two(&input);

    println!("Part 2: {}", part_two_answer);

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
    BoxLeft,
    BoxRight,
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
                        .flat_map(|(y, c)| match c {
                            '.' => [Object::Empty, Object::Empty],
                            '#' => [Object::Wall, Object::Wall],
                            'O' => [Object::BoxLeft, Object::BoxRight],
                            '@' => {
                                pos.x = x;
                                pos.y = y * 2;
                                [Object::Empty, Object::Empty]
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

fn part_two(input: &Input) -> usize {
    let mut pos = input.pos;
    let mut map = input.map.clone();

    input.moves.iter().for_each(|m| {
        let next = next_pos(&pos, &m);

        if can_move_into(&map, &next, &m) {
            move_existing(&mut map, &next, &m);
            pos = next;
        }
    });

    iter_2d(map.len(), map[0].len())
        .filter(|&(x, y)| match map[x][y] {
            Object::BoxLeft => true,
            _ => false,
        })
        .map(|(x, y)| 100 * x + y)
        .sum()
}

fn move_existing(map: &mut Map, move_from: &Pos, m: &Move) {
    match map[move_from.x][move_from.y] {
        Object::Wall => unreachable!("Wall"),
        Object::Empty => return,
        Object::BoxLeft | Object::BoxRight => {
            let move_to = next_pos(move_from, m);
            move_existing(map, &move_to, m);

            map[move_to.x][move_to.y] = map[move_from.x][move_from.y];
            map[move_from.x][move_from.y] = Object::Empty;

            match &m {
                Move::Up | Move::Down => match map[move_to.x][move_to.y] {
                    Object::BoxLeft => {
                        let right_move_from = next_pos(move_from, &Move::Right);
                        let right_move_to = next_pos(&move_to, &Move::Right);

                        move_existing(map, &right_move_to, m);
                        map[right_move_to.x][right_move_to.y] =
                            map[right_move_from.x][right_move_from.y];
                        map[right_move_from.x][right_move_from.y] = Object::Empty;
                    }
                    Object::BoxRight => {
                        let left_move_from = next_pos(move_from, &Move::Left);
                        let left_move_to = next_pos(&move_to, &Move::Left);

                        move_existing(map, &left_move_to, m);
                        map[left_move_to.x][left_move_to.y] =
                            map[left_move_from.x][left_move_from.y];
                        map[left_move_from.x][left_move_from.y] = Object::Empty;
                    }
                    _ => unreachable!("Unknown object"),
                },
                _ => {}
            }
        }
    };
}

fn can_move_into(map: &Map, move_from: &Pos, m: &Move) -> bool {
    match map[move_from.x][move_from.y] {
        Object::Wall => false,
        Object::Empty => true,
        Object::BoxLeft | Object::BoxRight => {
            let move_to = next_pos(move_from, m);

            match &m {
                Move::Left | Move::Right => can_move_into(map, &move_to, m),
                Move::Up | Move::Down => match map[move_from.x][move_from.y] {
                    Object::BoxLeft => {
                        let right_move_to = next_pos(&move_to, &Move::Right);
                        can_move_into(map, &move_to, m) && can_move_into(map, &right_move_to, m)
                    }

                    Object::BoxRight => {
                        let left_move_to = next_pos(&move_to, &Move::Left);
                        can_move_into(map, &left_move_to, m) && can_move_into(map, &move_to, m)
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
}

fn next_pos(pos: &Pos, m: &Move) -> Pos {
    match m {
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
    }
}

fn iter_2d(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |row| (0..width).map(move |col| (row, col)))
}
