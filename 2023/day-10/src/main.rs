use crate::input::Input;

mod input;

fn main() {
    let input = input::parse("example.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(_input: &Input) -> u32 {
    0
}

fn part_two(_input: &Input) -> u32 {
    0
}
