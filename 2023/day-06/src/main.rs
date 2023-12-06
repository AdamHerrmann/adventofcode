use input::parse_input_two;

use crate::input::parse_input_one;

mod input;
mod race;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u64 {
    parse_input_one("input.txt")
        .races
        .iter()
        .map(|r| r.ways_to_win())
        .product()
}

fn part_two() -> u64 {
    parse_input_two("input.txt").ways_to_win()
}
