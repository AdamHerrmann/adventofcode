use input::Direction;
use num::integer;
use std::{collections::HashSet, hash::Hash};

use crate::input::Input;

mod input;

fn main() {
    let input = Input::parse("input.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> u32 {
    let mut current = "AAA";

    input
        .directions
        .iter()
        .cycle()
        .take_while(|direction| {
            current = match direction {
                Direction::Left => input.left.get(current).unwrap(),
                Direction::Right => input.right.get(current).unwrap(),
            };

            current != "ZZZ"
        })
        .count() as u32
        + 1
}

#[derive(Eq, Clone, Copy)]
struct State<'a> {
    direction_number: usize,
    label: &'a str,
    step_number: usize,
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.direction_number == other.direction_number && self.label == other.label
    }
}

impl<'a> Hash for State<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.direction_number.hash(state);
        self.label.hash(state);
    }
}

// each cycle only contains one state where it stops.
// each cycle's offset matches its loop length so we can ignore it.
fn part_two(input: &Input) -> usize {
    input
        .elements
        .iter()
        .filter(|e| e.ends_with("A"))
        .map(|first| {
            let mut visited: HashSet<State> = HashSet::new();
            let mut current = first;

            let mut repeats_to = 0;
            let repeat_at = input
                .directions
                .iter()
                .enumerate()
                .cycle()
                .enumerate()
                .map(|(step_number, (direction_number, direction))| {
                    current = match direction {
                        Direction::Left => input.left.get(current).unwrap(),
                        Direction::Right => input.right.get(current).unwrap(),
                    };

                    State {
                        direction_number,
                        label: current,
                        step_number,
                    }
                })
                .take_while(|state| match visited.get(&state) {
                    Some(state) => {
                        repeats_to = state.step_number;
                        false
                    }
                    None => {
                        visited.insert(*state);
                        true
                    }
                })
                .count();

            repeat_at - repeats_to
        })
        .reduce(|acc, next| integer::lcm(acc, next))
        .unwrap()
}
