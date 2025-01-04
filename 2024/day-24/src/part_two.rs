use std::fmt::Debug;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

// const INPUT_FILE: &str = "./example_1.txt";
// const INPUT_FILE: &str = "./example_2.txt";
const INPUT_FILE: &str = "./input.txt";

type Input = Vec<Gate>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Wire([char; 3]);

impl Wire {
    fn parse(input: &str) -> Self {
        assert!(input.len() >= 3);
        Self([
            input.chars().nth(0).unwrap(),
            input.chars().nth(1).unwrap(),
            input.chars().nth(2).unwrap(),
        ])
    }

    fn numbered(prefix: char, number: usize) -> Self {
        assert!(number < 100);
        let first = ((number / 10) as u8 + b'0') as char;
        let second = ((number % 10) as u8 + b'0') as char;
        Self([prefix, first, second])
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Wire([a, b, c]) = self;
        write!(f, "{}{}{}", a, b, c)
    }
}

#[derive(Clone, Copy)]
struct Gate {
    left: Wire,
    right: Wire,
    out: Wire,
    operation: Operation,
}

impl Gate {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(" ");
        let left = Wire::parse(parts.next().unwrap());
        let operation = Operation::parse(parts.next().unwrap());
        let right = Wire::parse(parts.next().unwrap());
        assert_eq!(parts.next().unwrap(), "->");
        let out = Wire::parse(parts.next().unwrap());
        assert!(parts.next().is_none());

        Self {
            left,
            right,
            out,
            operation,
        }
    }

    fn swap_output(&mut self, a: Wire, b: Wire) {
        if self.out == a {
            self.out = b;
        } else if self.out == b {
            self.out = a;
        }
    }
}

impl Debug for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?} => {:?}",
            &self.left, &self.operation, &self.right, &self.out
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn parse(input: &str) -> Self {
        match input {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("Invalid operation"),
        }
    }
}

impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation_str = match self {
            Operation::And => "&",
            Operation::Or => "|",
            Operation::Xor => "^",
        };
        write!(f, "{}", operation_str)
    }
}

fn main() -> Result<()> {
    let input = parse_input()?;
    let part_two_answer = part_two(&input);
    println!("Part 2: {}", part_two_answer);
    Ok(())
}

fn parse_input() -> Result<Input> {
    let reader = BufReader::new(File::open(INPUT_FILE)?);
    Ok(reader
        .lines()
        .map(|l| l.unwrap())
        .skip_while(|l| l != "")
        .skip(1)
        .map(|l| Gate::parse(&l))
        .collect())
}

fn part_two(gates: &Input) -> usize {
    let mut gates = gates.clone();
    // z_0 = x_0 ^ y_0
    // C_0 = x_0 & y_0
    //
    // S_n = (x_n ^ y_n)
    // A_n = (x_n & y_n)
    // P_n = (C_n-1 & S_n)
    // C_n = A_n | P_n
    // z_n = S_n ^ C_n-1
    //
    // Z_N = A_N-1 | P_N-1

    // TODO: I could probably build this progamatically by looping through the
    // code below and applying swaps when we find issues.
    // But, I've already made a lot of assumption specific to my input so I'm
    // to move on for now.
    swap_wire_in_gates(&mut gates, Wire::parse("qff"), Wire::parse("qnw"));
    swap_wire_in_gates(&mut gates, Wire::parse("pbv"), Wire::parse("z16"));
    swap_wire_in_gates(&mut gates, Wire::parse("qqp"), Wire::parse("z23"));
    swap_wire_in_gates(&mut gates, Wire::parse("fbq"), Wire::parse("z36"));

    let xor_wires = (0..45)
        .map(|n| {
            find_gate(
                &gates,
                Wire::numbered('x', n),
                Wire::numbered('y', n),
                Operation::Xor,
            )
            .unwrap()
            .out
        })
        .collect::<Vec<_>>();
    let and_wires = (0..45)
        .map(|n| {
            find_gate(
                &gates,
                Wire::numbered('x', n),
                Wire::numbered('y', n),
                Operation::And,
            )
            .unwrap()
            .out
        })
        .collect::<Vec<_>>();

    let mut c_n_1 = find_gate(
        &gates,
        Wire::numbered('x', 0),
        Wire::numbered('y', 0),
        Operation::And,
    )
    .unwrap()
    .out;

    println!("A_0: {:?}", and_wires[0]);
    println!("C_0: {:?}", c_n_1);
    if xor_wires[0] != Wire::numbered('z', 0) {
        println!("Z_0 connected to wrong output");
        return 0;
    }
    println!();

    for n in 1..44 {
        println!("S_{}: {:?}", n, xor_wires[n]);
        println!("A_{}: {:?}", n, and_wires[n]);

        let p_n = find_gate(&gates, c_n_1, xor_wires[n], Operation::And);
        if p_n.is_none() {
            let candidate = gates
                .iter()
                .find(|g| {
                    g.operation == Operation::And
                        && (g.left == xor_wires[n] || g.right == xor_wires[n])
                })
                .or_else(|| {
                    gates.iter().find(|g| {
                        g.operation == Operation::And && (g.left == c_n_1 || g.right == c_n_1)
                    })
                });

            println!("Failed to find P_{}", n);
            println!("Expected: {:?} & {:?}", c_n_1, xor_wires[n]);
            println!("Candidate: {:?}", candidate);
            return 0;
        }
        let p_n = p_n.unwrap().out;
        println!("P_{}: {:?}", n, &p_n);

        let z_n = find_gate(&gates, xor_wires[n], c_n_1, Operation::Xor);
        if z_n.is_none() {
            println!("Failed to find Z_{}", n);
            return 0;
        }

        let z_n = z_n.unwrap().out;
        if z_n != Wire::numbered('z', n) {
            println!("Z_{} connected to wrong output", n);
            println!("Found: {:?}", z_n);
            return 0;
        }

        let c_n = find_gate(&gates, p_n, and_wires[n], Operation::Or);
        if c_n.is_none() {
            println!("Failed to find C_{}", n);
            return 0;
        }
        let c_n = c_n.unwrap().out;
        println!("C_{}: {:?}", n, &c_n_1);

        println!();

        c_n_1 = c_n;
    }

    0
}

fn find_gate(gates: &Vec<Gate>, left: Wire, right: Wire, operation: Operation) -> Option<&Gate> {
    gates.iter().find(|g| {
        g.operation == operation
            && ((g.left == left && g.right == right) || (g.left == right && g.right == left))
    })
}

fn swap_wire_in_gates(gates: &mut Vec<Gate>, a: Wire, b: Wire) {
    for gate in gates.iter_mut() {
        gate.swap_output(a, b);
    }
}
