use std::cmp;

use input::{parse, Input};
mod input;

fn main() {
    let input = parse("input.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> u32 {
    input
        .cards
        .iter()
        .map(|card| card.winning_numbers.intersection(&card.your_numbers))
        .map(|iter| iter.count())
        .map(|num_winning| match num_winning {
            0 => 0,
            n => 2_u32.pow((n as u32) - 1),
        })
        .sum()
}

fn part_two(input: &Input) -> u32 {
    let len = input.cards.len();
    let mut counts = vec![1; len];

    input
        .cards
        .iter()
        .map(|card| {
            card.winning_numbers
                .intersection(&card.your_numbers)
                .count()
        })
        .enumerate()
        .for_each(|(index, count)| {
            let start = cmp::min(index + 1, len);
            let end = cmp::min(index + count + 1, len);

            for i in start..end {
                counts[i] = counts[i] + counts[index];
            }
        });

    counts.iter().sum()
}
