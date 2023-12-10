use crate::input::Input;

mod input;

fn main() {
    let input = input::parse("input.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> i32 {
    input.readings.iter().map(compute_next).sum()
}

fn part_two(input: &Input) -> i32 {
    input.readings.iter().map(compute_previous).sum()
}

fn compute_next(reading: &Vec<i32>) -> i32 {
    let last_reading = *reading.last().unwrap();

    let diff = compute_diff(reading);
    if diff.iter().all(|d| *d == 0i32) {
        return last_reading;
    }

    last_reading + compute_next(&diff)
}

fn compute_previous(reading: &Vec<i32>) -> i32 {
    let first_reading = *reading.first().unwrap();

    let diff = compute_diff(reading);
    if diff.iter().all(|d| *d == 0i32) {
        return first_reading;
    }

    first_reading - compute_previous(&diff)
}

fn compute_diff(reading: &Vec<i32>) -> Vec<i32> {
    let mut iter = reading.iter();
    let mut previous = iter.next().unwrap();

    iter.map(|next| {
        let diff = next - previous;
        previous = next;
        diff
    })
    .collect()
}
