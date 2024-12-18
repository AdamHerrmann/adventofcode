use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example_1.txt";
// const INPUT_FILE: &str = "./example_2.txt";
// const INPUT_FILE: &str = "./example_3.txt";
// const INPUT_FILE: &str = "./example_4.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<Vec<char>>;

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
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Input>())
}

type RegionMap = Vec<Vec<Option<u16>>>;

fn part_one(input: &Input) -> usize {
    let height = input.len();
    let width = input[0].len();

    let (region_map, region_count) = build_region_map(input);

    let mut perimeter = vec![0; region_count as usize];
    let mut area = vec![0; region_count as usize];

    for (x, y) in iter_2d(height, width) {
        let region_id = region_map[x][y].unwrap() as usize;
        area[region_id] += 1;

        let n = neighbors(x, y, height, width);
        perimeter[region_id] += 4 - n.len() as u8;

        for (nx, ny) in n {
            if region_map[nx][ny].unwrap() != region_id as u16 {
                perimeter[region_id] += 1;
            }
        }
    }

    area.iter()
        .zip(perimeter.iter())
        .map(|(&a, &p)| a as usize * p as usize)
        .sum()
}

fn iter_2d(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |row| (0..width).map(move |col| (row, col)))
}

fn build_region_map(input: &Input) -> (RegionMap, u16) {
    let height = input.len();
    let width = input[0].len();
    let mut region_map: RegionMap = vec![vec![None; width]; height];
    let mut region_id = 0;

    for (x, y) in iter_2d(height, width) {
        if region_map[x][y].is_none() {
            build_region_map_from(&mut region_map, input, x, y, region_id);
            region_id += 1;
        }
    }

    (region_map, region_id)
}

fn build_region_map_from(
    region_map: &mut RegionMap,
    input: &Input,
    x: usize,
    y: usize,
    region_id: u16,
) {
    let height = input.len();
    let width = input[0].len();

    let mut stack = vec![(x, y)];

    while let Some((x, y)) = stack.pop() {
        if region_map[x][y].is_some() {
            continue;
        }

        region_map[x][y] = Some(region_id);

        for (nx, ny) in neighbors(x, y, height, width) {
            if input[nx][ny] == input[x][y] {
                stack.push((nx, ny));
            }
        }
    }
}

fn neighbors(x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push((x - 1, y));
    }

    if x < height - 1 {
        neighbors.push((x + 1, y));
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }

    if y < width - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn part_two(input: &Input) -> usize {
    let height = input.len();
    let width = input[0].len();

    let (region_map, region_count) = build_region_map(input);

    let mut sides = vec![0; region_count as usize];
    let mut area = vec![0; region_count as usize];

    for (x, y) in iter_2d(height, width) {
        let region_id = region_map[x][y].unwrap() as usize;
        area[region_id] += 1;

        // number of sides is equal to the number of corners
        // a corner is a cell where perpendicular neighbor's equality to the
        // current region XOR to false
        let north = x > 0 && region_map[x - 1][y].unwrap() == region_id as u16;
        let north_east =
            x > 0 && y < width - 1 && region_map[x - 1][y + 1].unwrap() == region_id as u16;
        let east = y < width - 1 && region_map[x][y + 1].unwrap() == region_id as u16;
        let south_east = x < height - 1
            && y < width - 1
            && region_map[x + 1][y + 1].unwrap() == region_id as u16;
        let south = x < height - 1 && region_map[x + 1][y].unwrap() == region_id as u16;
        let south_west =
            x < height - 1 && y > 0 && region_map[x + 1][y - 1].unwrap() == region_id as u16;
        let west = y > 0 && region_map[x][y - 1].unwrap() == region_id as u16;
        let north_west = x > 0 && y > 0 && region_map[x - 1][y - 1].unwrap() == region_id as u16;

        // XX  0X  X0  00  XX  0X  X0  00
        // XX  XX  XX  XX  0X  0X  0X  0X
        //  0   1   0   0   0   0   1   1

        if (!west && !north) || (west && north && !north_west) {
            sides[region_id] += 1;
        }

        if (!north && !east) || (north && east && !north_east) {
            sides[region_id] += 1;
        }

        if (!east && !south) || (east && south && !south_east) {
            sides[region_id] += 1;
        }

        if (!south && !west) || (south && west && !south_west) {
            sides[region_id] += 1;
        }
    }

    print_region_map(&region_map);
    area.iter()
        .zip(sides.iter())
        .enumerate()
        .for_each(|(i, (&a, &p))| {
            println!("Region {}: area: {}, sides: {}", i, a, p);
        });

    area.iter()
        .zip(sides.iter())
        .map(|(&a, &p)| a as usize * p as usize)
        .sum()
}

fn print_region_map(region_map: &RegionMap) {
    for row in region_map {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Some(id) => id.to_string(),
                    None => ".".to_string(),
                }
            );
        }
        println!();
    }
}
