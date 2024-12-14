use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

#[derive(Debug)]
struct Input {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
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
    let mut rules = HashMap::new();
    let mut updates = Vec::new();
    let mut processing_updates = false;

    reader.lines().for_each(|line| {
        let line = line.unwrap();

        if line.is_empty() {
            processing_updates = true;
            return;
        }

        if processing_updates {
            updates.push(
                line.split(",")
                    .map(|p| p.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
        } else {
            let before = line[0..2].parse::<u32>().unwrap();
            let after = line[3..5].parse::<u32>().unwrap();

            rules
                .entry(before)
                .and_modify(|s: &mut HashSet<u32>| {
                    s.insert(after);
                })
                .or_insert_with(|| HashSet::from([after]));
        }
    });

    Ok(Input { rules, updates })
}

fn part_one(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| is_valid_update(&input.rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_valid_update(rules: &HashMap<u32, HashSet<u32>>, update: &Vec<u32>) -> bool {
    update
        .iter()
        .enumerate()
        .all(|(i, page)| page_can_be_after(rules, *page, &update[0..i]))
}

fn page_can_be_after(rules: &HashMap<u32, HashSet<u32>>, page: u32, earlier_pages: &[u32]) -> bool {
    let rules = rules.get(&page);
    if rules.is_none() {
        return true;
    }

    let page_must_be_before = rules.unwrap();
    !earlier_pages
        .iter()
        .any(|earlier_page| page_must_be_before.contains(earlier_page))
}

fn part_two(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| !is_valid_update(&input.rules, update))
        .map(|update| reorder_update(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn reorder_update(update: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut ordered = update.clone();

    for ordered_length in 0..update.len() {
        let mut page_index = ordered_length;
        while page_index < update.len()
            && !page_can_be_after(rules, ordered[page_index], &ordered[ordered_length..])
        {
            page_index += 1;
        }

        if page_index != ordered_length {
            ordered.swap(ordered_length, page_index);
        }
    }

    // NOTE: technically we've ordered them backwards, but we're only interested in the middle page so don't bother reversing
    ordered
}
