use input::{parse, Input, Number, Symbol};

mod input;
mod parser;

fn main() {
    let input = parse("input.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> u32 {
    input
        .numbers
        .iter()
        .filter(|number| {
            input
                .symbols
                .iter()
                .any(|symbol| is_adjacent(number, symbol))
        })
        .map(|number| number.value)
        .sum()
}

fn part_two(input: &Input) -> u32 {
    input
        .symbols
        .iter()
        .filter(|symbol| symbol.value == '*')
        .map(|symbol| {
            input
                .numbers
                .iter()
                .filter(|number| is_adjacent(number, symbol))
                .take(3)
        })
        .map(|t| t.collect())
        .filter(|t: &Vec<&Number>| t.len() == 2)
        .map(|t| t.iter().map(|n| n.value).product::<u32>())
        .sum()
}

fn is_adjacent(number: &Number, symbol: &Symbol) -> bool {
    if symbol.loc.column > number.end.column || symbol.loc.line > number.start.line + 1 {
        return false;
    }

    let line_min = match number.start.line {
        0 => 0,
        l => l - 1,
    };
    let column_min = match number.start.column {
        0 => 0,
        c => c - 1,
    };

    line_min <= symbol.loc.line && column_min <= symbol.loc.column
}
