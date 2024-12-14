use std::cmp;
use std::io::prelude::*;
use std::ops::Range;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example.txt";
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
    let board = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    Ok(board)
}

fn iter_2d(row: Range<usize>, col: Range<usize>) -> impl Iterator<Item = (usize, usize)> {
    row.flat_map(move |row| col.clone().map(move |col| (row, col)))
}

fn part_one(input: &Input) -> usize {
    let height = input.len();
    let width = input[0].len();

    iter_2d(3..height, 0..width)
        .filter(|&(row, column)| xmas_north(input, row, column))
        .count()
        + iter_2d(3..height, 0..width - 3)
            .filter(|&(row, column)| xmas_north_east(input, row, column))
            .count()
        + iter_2d(0..height, 0..width - 3)
            .filter(|&(row, column)| xmas_east(input, row, column))
            .count()
        + iter_2d(0..height - 3, 0..width - 3)
            .filter(|&(row, column)| xmas_south_east(input, row, column))
            .count()
        + iter_2d(0..height - 3, 0..width)
            .filter(|&(row, column)| xmas_south(input, row, column))
            .count()
        + iter_2d(0..height - 3, 3..width)
            .filter(|&(row, column)| xmas_south_west(input, row, column))
            .count()
        + iter_2d(0..height, 3..width)
            .filter(|&(row, column)| xmas_west(input, row, column))
            .count()
        + iter_2d(3..height, 3..width)
            .filter(|&(row, column)| xmas_north_west(input, row, column))
            .count()
}

fn xmas_north(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && board[row - 1][column] == 'M'
        && board[row - 2][column] == 'A'
        && board[row - 3][column] == 'S'
}

fn xmas_north_east(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && board[row - 1][column + 1] == 'M'
        && board[row - 2][column + 2] == 'A'
        && board[row - 3][column + 3] == 'S'
}

fn xmas_east(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && board[row][column + 1] == 'M'
        && board[row][column + 2] == 'A'
        && board[row][column + 3] == 'S'
}

fn xmas_south_east(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && column < board[row].len() - 3
        && board[row + 1][column + 1] == 'M'
        && board[row + 2][column + 2] == 'A'
        && board[row + 3][column + 3] == 'S'
}

fn xmas_south(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && board[row + 1][column] == 'M'
        && board[row + 2][column] == 'A'
        && board[row + 3][column] == 'S'
}

fn xmas_south_west(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && board[row + 1][column - 1] == 'M'
        && board[row + 2][column - 2] == 'A'
        && board[row + 3][column - 3] == 'S'
}

fn xmas_west(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && board[row][column - 1] == 'M'
        && board[row][column - 2] == 'A'
        && board[row][column - 3] == 'S'
}

fn xmas_north_west(board: &Input, row: usize, column: usize) -> bool {
    board[row][column] == 'X'
        && board[row - 1][column - 1] == 'M'
        && board[row - 2][column - 2] == 'A'
        && board[row - 3][column - 3] == 'S'
}

fn part_two(input: &Input) -> usize {
    let height = input.len();
    let width = input[0].len();

    iter_2d(1..height - 1, 1..width - 1)
        .filter(|&(row, column)| x_mas(input, row, column))
        .count()
}

fn x_mas(board: &Input, row: usize, col: usize) -> bool {
    board[row][col] == 'A'
        && ((board[row - 1][col - 1] == 'M' && board[row + 1][col + 1] == 'S')
            || (board[row - 1][col - 1] == 'S' && board[row + 1][col + 1] == 'M'))
        && ((board[row - 1][col + 1] == 'M' && board[row + 1][col - 1] == 'S')
            || (board[row - 1][col + 1] == 'S' && board[row + 1][col - 1] == 'M'))
}
