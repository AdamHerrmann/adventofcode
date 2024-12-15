use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<DiskBlock>;

#[derive(Clone, Debug)]
struct DiskBlock {
    block_type: DiskBlockType,
    length: u8,
}

#[derive(Clone, Debug)]
enum DiskBlockType {
    Free,
    File(u16),
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

    Ok(reader
        .bytes()
        .map(|b| b.unwrap())
        .filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            _ => None,
        })
        .enumerate()
        .map(|(index, length)| match index % 2 {
            0 => DiskBlock {
                block_type: DiskBlockType::File(index as u16 / 2),
                length,
            },
            1 => DiskBlock {
                block_type: DiskBlockType::Free,
                length,
            },
            _ => unreachable!(),
        })
        .collect::<Input>())
}

struct CompactedDiskIter<'a> {
    entries: &'a Input,
    forward_index: usize,
    forward_offset: u8,
    reverse_index: usize,
    reverse_offset: u8,
}

impl<'a> CompactedDiskIter<'a> {
    fn new(entries: &'a Input) -> Self {
        let last = entries.len() - 1;
        let reverse_index = match &entries[last].block_type {
            DiskBlockType::File(_id) => last,
            DiskBlockType::Free => last - 1,
        };

        Self {
            entries,
            forward_index: 0,
            forward_offset: 0,
            reverse_index,
            reverse_offset: 0,
        }
    }
}

impl<'a> Iterator for CompactedDiskIter<'a> {
    type Item = &'a u16;

    fn next(&mut self) -> Option<Self::Item> {
        let reverse_length = self.entries[self.reverse_index].length;
        if self.reverse_offset == reverse_length {
            self.reverse_offset = 0;
            self.reverse_index -= 2;

            return self.next();
        }

        let forward_length = self.entries[self.forward_index].length;
        if self.forward_offset == forward_length {
            self.forward_index += 1;

            if self.forward_index == self.reverse_index {
                self.forward_offset = self.reverse_offset;
            } else {
                self.forward_offset = 0;
            }

            return self.next();
        }

        if self.forward_index > self.reverse_index {
            return None;
        }

        self.forward_offset += 1;

        match &self.entries[self.forward_index].block_type {
            DiskBlockType::File(id) => Some(id),

            DiskBlockType::Free => match &self.entries[self.reverse_index].block_type {
                DiskBlockType::File(id) => {
                    self.reverse_offset += 1;
                    Some(id)
                }
                _ => unreachable!(),
            },
        }
    }
}

fn part_one(input: &Input) -> usize {
    CompactedDiskIter::new(input)
        .enumerate()
        .map(|(index, id)| index * (*id as usize))
        .sum()
}

struct DiskIter<'a> {
    entries: &'a Input,
    index: usize,
    offset: u8,
}

impl<'a> DiskIter<'a> {
    pub fn new(entries: &'a Input) -> Self {
        Self {
            entries,
            index: 0,
            offset: 0,
        }
    }
}

impl<'a> Iterator for DiskIter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.entries.len() {
            return None;
        }

        if self.offset == self.entries[self.index].length {
            self.offset = 0;
            self.index += 1;

            return self.next();
        }

        self.offset += 1;
        match &self.entries[self.index].block_type {
            DiskBlockType::File(id) => Some(*id),
            DiskBlockType::Free => Some(0),
        }
    }
}

fn part_two(input: &Input) -> usize {
    let mut entries = input.clone();

    let mut compacted_from = entries.len();
    while compacted_from != 0 {
        compacted_from -= 1;

        match entries[compacted_from].block_type {
            DiskBlockType::Free => (),
            DiskBlockType::File(_id) => {
                // try to find a place this block an go
                let insert_at = (0..compacted_from).find(|index| {
                    let block = &entries[*index];

                    if block.length < entries[compacted_from].length {
                        return false;
                    }

                    match block.block_type {
                        DiskBlockType::Free => true,
                        DiskBlockType::File(_) => false,
                    }
                });

                if let Some(insert_at) = insert_at {
                    let file_block = entries.remove(compacted_from);

                    entries[insert_at].length -= file_block.length;
                    entries[compacted_from - 1].length += file_block.length;

                    entries.insert(insert_at, file_block);
                }
            }
        }
    }

    DiskIter::new(&entries)
        .enumerate()
        .map(|(index, id)| index * id as usize)
        .sum()
}
