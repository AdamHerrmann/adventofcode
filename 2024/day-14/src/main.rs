use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;
use regex::Regex;

// const INPUT_FILE: &str = "./example.txt";
// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;
// const TIME: i32 = 100;

const INPUT_FILE: &str = "./input.txt";
const WIDTH: u32 = 101;
const HEIGHT: u32 = 103;
const TIME: u32 = 100;

const X_MIDDLE: u32 = WIDTH / 2;
const Y_MIDDLE: u32 = HEIGHT / 2;

type Input = Vec<Robot>;

struct Robot {
    x: u32,
    y: u32,
    dx: u32,
    dy: u32,
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
    let line_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let height = HEIGHT as i32;
    let width = WIDTH as i32;

    Ok(reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, [x, y, dx, dy]) = line_regex.captures(&line).unwrap().extract();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let dx = dx.parse::<i32>().unwrap();
            let dy = dy.parse::<i32>().unwrap();

            let dx = (if dx < 0 {
                dx % width + width
            } else {
                dx % width
            }) as u32;
            let dy = (if dy < 0 {
                dy % height + height
            } else {
                dy % height
            }) as u32;

            Robot { x, y, dx, dy }
        })
        .collect::<Input>())
}

fn part_one(input: &Input) -> usize {
    let mut quadrants = [0, 0, 0, 0];

    input
        .iter()
        .map(|r| ((r.x + r.dx * TIME) % WIDTH, (r.y + r.dy * TIME) % HEIGHT))
        .fold(&mut quadrants, |quadrants, (x, y)| {
            if x == X_MIDDLE || y == Y_MIDDLE {
                return quadrants;
            }

            match (x < X_MIDDLE, y < Y_MIDDLE) {
                (true, true) => quadrants[0] += 1,
                (true, false) => quadrants[1] += 1,
                (false, true) => quadrants[2] += 1,
                (false, false) => quadrants[3] += 1,
            };

            quadrants
        })
        .iter()
        .product()
}

fn part_two(input: &Input) -> u32 {
    let time = (0..1000000)
        .find(|time| {
            let mut points = input
                .iter()
                .map(|r| ((r.x + r.dx * time) % WIDTH, (r.y + r.dy * time) % HEIGHT))
                .collect::<Vec<_>>();
            points.sort();

            let mut count = 0;
            points.iter().reduce(|prev, curr| {
                let diff = prev.0.abs_diff(curr.0) + prev.1.abs_diff(curr.1);

                if diff <= 2 {
                    count += 1;
                }

                curr
            });

            count > input.len() / 2
        })
        .unwrap();

    let mut map = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
    input
        .iter()
        .map(|r| ((r.x + r.dx * time) % WIDTH, (r.y + r.dy * time) % HEIGHT))
        .for_each(|(x, y)| {
            map[y as usize][x as usize] = '#';
        });

    for row in map {
        println!("{}", row.iter().collect::<String>());
    }

    time
}
