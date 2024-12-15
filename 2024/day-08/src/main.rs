use num::integer::gcd;
use std::cmp;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
const INPUT_FILE: &str = "./input.txt";

struct Input {
    height: usize,
    width: usize,
    antennas: HashMap<char, Vec<(isize, isize)>>,
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

    let mut height = 0;
    let mut width = 0;
    let mut antennas = HashMap::<char, Vec<(isize, isize)>>::new();

    for (x, lines) in reader.lines().map(|l| l.unwrap()).enumerate() {
        height += 1;

        for (y, c) in lines.chars().enumerate() {
            width = cmp::max(width, y + 1); // :(
            match c {
                '.' => {}
                frequency => {
                    antennas
                        .entry(frequency)
                        .or_insert(Vec::new())
                        .push((x as isize, y as isize));
                }
            }
        }
    }

    Ok(Input {
        height,
        width,
        antennas,
    })
}

fn part_one(input: &Input) -> usize {
    let height = input.height;
    let width = input.width;
    let antennas = &input.antennas;
    let mut antinodes = vec![vec![false; width]; height];

    for positions in antennas.values() {
        for (offset, (x1, y1)) in positions.iter().enumerate() {
            for (x2, y2) in positions.iter().skip(offset + 1) {
                let dx = x2 - x1;
                let dy = y2 - y1;

                let ax = x1 - dx;
                let ay = y1 - dy;
                if ax >= 0 && ax < height as isize && ay >= 0 && ay < width as isize {
                    antinodes[ay as usize][ax as usize] = true;
                }

                let bx = x2 + dx;
                let by = y2 + dy;
                if bx >= 0 && bx < height as isize && by >= 0 && by < width as isize {
                    antinodes[by as usize][bx as usize] = true;
                }
            }
        }
    }

    antinodes.iter().flatten().filter(|&&b| b).count()
}

fn part_two(input: &Input) -> usize {
    let height = input.height;
    let width = input.width;
    let antennas = &input.antennas;
    let mut antinodes = vec![vec![false; width]; height];

    for positions in antennas.values() {
        for (offset, (x1, y1)) in positions.iter().enumerate() {
            for (x2, y2) in positions.iter().skip(offset + 1) {
                let dx = x2 - x1;
                let dy = y2 - y1;
                let d = gcd(dx, dy);

                let dx = dx / d;
                let dy = dy / d;

                let mut ax = *x1;
                let mut ay = *y1;
                while ax >= 0 && ax < height as isize && ay >= 0 && ay < width as isize {
                    antinodes[ax as usize][ay as usize] = true;

                    ax -= dx;
                    ay -= dy;
                }

                ax = x1 + dx;
                ay = y1 + dy;
                while ax >= 0 && ax < height as isize && ay >= 0 && ay < width as isize {
                    antinodes[ax as usize][ay as usize] = true;

                    ax += dx;
                    ay += dy;
                }
            }
        }
    }

    antinodes.iter().flatten().filter(|&&b| b).count()
}
