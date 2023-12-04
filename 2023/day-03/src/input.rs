use std::{fs::File, io::BufReader};
use utf8_chars::BufReadCharsExt;

use crate::parser::{Location, Parser};

pub struct Number {
    pub start: Location,
    pub end: Location,
    pub value: u32,
}

pub struct Symbol {
    pub loc: Location,
    pub value: char,
}

pub struct Input {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}

pub fn parse(filename: &str) -> Input {
    let file = File::open(filename).expect("Failed to open input file");
    let mut reader = BufReader::new(file);
    let iter = reader.chars().map(|c| c.unwrap());

    let mut parser = Parser::new(iter);
    let mut result = Input {
        numbers: Vec::new(),
        symbols: Vec::new(),
    };

    loop {
        loop {
            match parser.peek() {
                Some('0'..='9') => break,
                Some('.' | '\n') => parser.consume(),
                Some(value) => {
                    result.symbols.push(Symbol {
                        loc: parser.loc(),
                        value,
                    });
                    parser.consume();
                }
                None => return result,
            }
        }

        let start = parser.loc();
        let mut value = 0;

        loop {
            match parser.peek() {
                Some(d @ '0'..='9') => {
                    value = value * 10 + d.to_digit(10).unwrap();
                    parser.consume();
                }
                _ => {
                    result.numbers.push(Number {
                        start,
                        end: parser.loc(),
                        value,
                    });
                    break;
                }
            }
        }
    }
}
