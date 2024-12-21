use bitvec::vec::BitVec;
use keyed_priority_queue::{Entry, KeyedPriorityQueue};
use std::cmp::Reverse;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example_1.txt";
// const INPUT_FILE: &str = "./example_2.txt";
const INPUT_FILE: &str = "./input.txt";

struct Input {
    map: Map,
    start: Position,
    end: Position,
}

type Map = Vec<Vec<Object>>;
enum Object {
    Empty,
    Wall,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct State {
    position: Position,
    facing: Facing,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    pub fn index(&self) -> usize {
        match self {
            Facing::North => 0,
            Facing::East => 1,
            Facing::South => 2,
            Facing::West => 3,
        }
    }
}

type Cost = u32;
const MOVE_COST: Cost = 1;
const TURN_COST: Cost = 1000;

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

    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };

    let map = reader
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '.' => Object::Empty,
                    '#' => Object::Wall,
                    'S' => {
                        start.x = x;
                        start.y = y;
                        Object::Empty
                    }
                    'E' => {
                        end.x = x;
                        end.y = y;
                        Object::Empty
                    }
                    _ => panic!("Invalid character: {}", c),
                })
                .collect::<Vec<Object>>()
        })
        .collect::<Map>();

    Ok(Input { map, start, end })
}

#[derive(Clone)]
struct Visited {
    visited: BitVec,
    width: usize,
}

impl Visited {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            visited: BitVec::repeat(false, height * width),
            width,
        }
    }

    fn index(&self, position: &Position) -> usize {
        let x = position.x;
        let y = position.y;
        x * self.width + y
    }

    pub fn contains(&self, position: &Position) -> bool {
        self.visited[self.index(position)]
    }

    pub fn insert(&mut self, position: &Position) -> bool {
        let index = self.index(position);
        let prev = self.visited[index];
        self.visited.set(index, true);
        !prev
    }

    pub fn or_assign(&mut self, other: &Self) {
        self.visited |= &other.visited;
    }
}

fn part_one(input: &Input) -> Cost {
    let height = input.map.len();
    let width = input.map[0].len();
    let mut queue = KeyedPriorityQueue::<State, Reverse<Cost>>::new();
    let mut visited = vec![Visited::new(width, height); 4];

    let start_state = State {
        position: input.start,
        facing: Facing::East,
    };
    queue.push(start_state, Reverse(0));

    while let Some((state, Reverse(cost))) = queue.pop() {
        if state.position == input.end {
            return cost;
        }

        if !visited[state.facing.index()].insert(&state.position) {
            continue;
        }

        let move_state = create_move_state(&state);
        if !visited[move_state.facing.index()].contains(&move_state.position)
            && !is_wall(&input.map, &move_state)
        {
            push_queue(&mut queue, move_state, cost + MOVE_COST);
        }

        let left_turn_state = create_left_turn_state(&state);
        if !visited[left_turn_state.facing.index()].contains(&left_turn_state.position) {
            push_queue(&mut queue, left_turn_state, cost + TURN_COST);
        }

        let right_turn_state = create_right_turn_state(&state);
        if !visited[right_turn_state.facing.index()].contains(&right_turn_state.position) {
            push_queue(&mut queue, right_turn_state, cost + TURN_COST);
        }
    }

    unreachable!("Failed to find path")
}

fn part_two(input: &Input) -> usize {
    let height = input.map.len();
    let width = input.map[0].len();
    let mut queue = KeyedPriorityQueue::<State, Reverse<Cost>>::new();
    let mut visited = vec![Visited::new(width, height); 4];
    let mut best_paths = vec![vec![vec![Visited::new(width, height); 4]; width]; height];

    let start_state = State {
        position: input.start,
        facing: Facing::East,
    };
    queue.push(start_state, Reverse(0));
    best_paths[input.start.x][input.start.y][start_state.facing.index()]
        .insert(&start_state.position);

    while let Some((state, Reverse(cost))) = queue.pop() {
        if state.position == input.end {
            break;
        }

        if !visited[state.facing.index()].insert(&state.position) {
            continue;
        }

        let move_state = create_move_state(&state);
        if !visited[move_state.facing.index()].contains(&move_state.position)
            && !is_wall(&input.map, &move_state)
        {
            push_queue_and_paths(
                &mut queue,
                &mut best_paths,
                &state,
                move_state,
                cost + MOVE_COST,
            );
        }

        let left_turn_state = create_left_turn_state(&state);
        if !visited[left_turn_state.facing.index()].contains(&left_turn_state.position) {
            push_queue_and_paths(
                &mut queue,
                &mut best_paths,
                &state,
                left_turn_state,
                cost + TURN_COST,
            );
        }

        let right_turn_state = create_right_turn_state(&state);
        if !visited[right_turn_state.facing.index()].contains(&right_turn_state.position) {
            push_queue_and_paths(
                &mut queue,
                &mut best_paths,
                &state,
                right_turn_state,
                cost + TURN_COST,
            );
        }
    }

    best_paths[input.end.x][input.end.y]
        .iter()
        .map(|v| v.visited.clone())
        .reduce(|a, b| a | b)
        .unwrap()
        .count_ones() as usize
}

fn create_move_state(state: &State) -> State {
    let mut position = state.position;
    match state.facing {
        Facing::North => position.x -= 1,
        Facing::East => position.y += 1,
        Facing::South => position.x += 1,
        Facing::West => position.y -= 1,
    }
    State {
        position,
        facing: state.facing,
    }
}

fn create_left_turn_state(state: &State) -> State {
    let facing = match state.facing {
        Facing::North => Facing::West,
        Facing::East => Facing::North,
        Facing::South => Facing::East,
        Facing::West => Facing::South,
    };
    State {
        position: state.position,
        facing,
    }
}

fn create_right_turn_state(state: &State) -> State {
    let facing = match state.facing {
        Facing::North => Facing::East,
        Facing::East => Facing::South,
        Facing::South => Facing::West,
        Facing::West => Facing::North,
    };
    State {
        position: state.position,
        facing,
    }
}

fn is_wall(map: &Map, state: &State) -> bool {
    match map[state.position.x][state.position.y] {
        Object::Empty => false,
        Object::Wall => true,
    }
}

fn push_queue(queue: &mut KeyedPriorityQueue<State, Reverse<Cost>>, state: State, cost: Cost) {
    let cost = Reverse(cost);
    match queue.entry(state) {
        Entry::Occupied(entry) => {
            if *entry.get_priority() < cost {
                entry.set_priority(cost);
            }
        }
        Entry::Vacant(entry) => {
            entry.set_priority(cost);
        }
    }
}

fn push_queue_and_paths(
    queue: &mut KeyedPriorityQueue<State, Reverse<Cost>>,
    paths: &mut Vec<Vec<Vec<Visited>>>,
    previous: &State,
    state: State,
    cost: Cost,
) {
    let cost = Reverse(cost);
    match queue.entry(state) {
        Entry::Occupied(entry) => {
            let previous_cost = *entry.get_priority();

            if previous_cost < cost {
                replace_paths(paths, previous, &state);
                entry.set_priority(cost);
            } else if previous_cost == cost {
                update_paths(paths, previous, &state);
            }
        }
        Entry::Vacant(entry) => {
            entry.set_priority(cost);
            replace_paths(paths, previous, &state);
        }
    }
}

fn replace_paths(paths: &mut Vec<Vec<Vec<Visited>>>, previous: &State, state: &State) {
    assert!(previous != state);

    let previous_x = previous.position.x;
    let previous_y = previous.position.y;
    let previous_facing = previous.facing.index();

    let state_x = state.position.x;
    let state_y = state.position.y;
    let state_facing = state.facing.index();

    paths[state_x][state_y][state_facing] = paths[previous_x][previous_y][previous_facing].clone();
    paths[state_x][state_y][state.facing.index()].insert(&state.position);
}

fn update_paths(paths: &mut Vec<Vec<Vec<Visited>>>, previous: &State, state: &State) {
    assert!(previous != state);

    let previous_x = previous.position.x;
    let previous_y = previous.position.y;
    let previous_facing = previous.facing.index();

    let state_x = state.position.x;
    let state_y = state.position.y;
    let state_facing = state.facing.index();

    paths[state_x][state_y][state_facing].insert(&state.position);

    if state_x != previous_x {
        if state_x < previous_x {
            let (state, previous) = paths.split_at_mut(previous_x);
            state[state_x][state_y][state_facing]
                .or_assign(&previous[0][previous_y][previous_facing]);
        } else {
            let (previous, state) = paths.split_at_mut(state_x);
            state[0][state_y][state_facing]
                .or_assign(&previous[previous_x][previous_y][previous_facing]);
        }

        return;
    }

    if state_y != previous_y {
        if state_y < previous_y {
            let (state, previous) = paths[state_x].split_at_mut(previous_y);
            state[state_y][state_facing].or_assign(&previous[0][previous_facing]);
        } else {
            let (previous, state) = paths[state_x].split_at_mut(state_y);
            state[0][state_facing].or_assign(&previous[previous_y][previous_facing]);
        }

        return;
    }

    if state_facing < previous_facing {
        let (state, previous) = paths[state_x][state_y].split_at_mut(previous_facing);
        state[state_facing].or_assign(&previous[0]);
    } else {
        let (previous, state) = paths[state_x][state_y].split_at_mut(state_facing);
        state[0].or_assign(&previous[previous_facing]);
    }
}
