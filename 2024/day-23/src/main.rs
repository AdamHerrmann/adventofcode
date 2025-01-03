use std::collections::BinaryHeap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;
use bitvec::{bitarr, BitArr};
use smallvec::SmallVec;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

const A: u8 = 'a' as u8;
const LETTER_RADIX: usize = 26;
const MAX_NODES: usize = LETTER_RADIX * LETTER_RADIX;
type NodeSet = BitArr![for MAX_NODES];
type Input = [NodeSet; MAX_NODES];

struct Partition {
    nodes: NodeSet,
    prefix: usize,
    size: usize,
}

impl Partition {
    fn all_connected(&self, input: &Input) -> bool {
        self.nodes
            .iter_ones()
            .fold(bitarr![1; MAX_NODES], |acc, index| acc & input[index])
            == self.nodes
    }

    fn to_string(&self) -> String {
        self.nodes
            .iter_ones()
            .map(|index| index_to_chars(index))
            .map(|(a, b)| format!("{}{}", a, b))
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl PartialEq for Partition {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl Eq for Partition {}

impl PartialOrd for Partition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.size.cmp(&other.size))
    }
}

impl Ord for Partition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
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
    let mut input = [bitarr![0; MAX_NODES]; MAX_NODES];

    for line in reader.lines() {
        let line = line?;
        let chars = line.chars().collect::<SmallVec<[char; 5]>>();
        assert_eq!(chars.len(), 5);

        let first = chars_to_index(chars[0], chars[1]);
        let second = chars_to_index(chars[3], chars[4]);

        input[first].set(first, true);
        input[first].set(second, true);
        input[second].set(first, true);
        input[second].set(second, true);
    }

    Ok(input)
}

fn chars_to_index(a: char, b: char) -> usize {
    let a = (a as u8 - A) as usize;
    let b = (b as u8 - A) as usize;

    a * LETTER_RADIX + b
}

fn index_to_chars(index: usize) -> (char, char) {
    let a = (index / LETTER_RADIX) as u8 + A;
    let b = (index % LETTER_RADIX) as u8 + A;

    (a as char, b as char)
}

fn part_one(input: &Input) -> usize {
    let t_range = chars_to_index('t', 'a')..chars_to_index('u', 'a');

    t_range
        .clone()
        .map(|first| {
            input[first]
                .iter_ones()
                .filter(|&second| !t_range.contains(&second) || second < first)
                .map(|second| {
                    (input[first] & input[second])
                        .iter_ones()
                        .take_while(|&third| third < second)
                        .filter(|&third| !t_range.contains(&third) || third < first)
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_two(input: &Input) -> String {
    let first = {
        let mut all = bitarr![0; MAX_NODES];
        for (index, adj) in input.iter().enumerate() {
            all.set(index, adj.any());
        }
        Partition {
            nodes: all,
            prefix: 0,
            size: all.count_ones() as usize,
        }
    };

    let mut queue = BinaryHeap::new();
    queue.push(first);

    while let Some(partition) = queue.pop() {
        if partition.all_connected(input) {
            return partition.to_string();
        }

        let partition_on = {
            let (_, suffix) = partition.nodes.split_at(partition.prefix);
            suffix.first_one().unwrap() + partition.prefix
        };

        let mut connected = Partition {
            nodes: bitarr![0; MAX_NODES],
            prefix: partition_on + 1,
            size: 0,
        };
        let mut unconnected = Partition {
            nodes: bitarr![0; MAX_NODES],
            prefix: partition_on + 1,
            size: 0,
        };

        for index in partition.nodes.iter_ones() {
            if index == partition_on || *input[index].get(partition_on).unwrap() {
                connected.nodes.set(index, true);
                connected.size += 1;
            } else {
                unconnected.nodes.set(index, true);
                unconnected.size += 1;
            }
        }

        if connected.size > 0 {
            queue.push(connected);
        }

        if unconnected.size > 0 {
            queue.push(unconnected);
        }
    }

    unreachable!("Failed to find any partition");
}
