use std::fmt::Debug;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use anyhow::Result;

type Input = Computer;

#[derive(Clone, Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

#[derive(Debug)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

#[derive(Debug)]
struct LiteralOperand(u8);
impl LiteralOperand {
    fn value(&self) -> u64 {
        assert!(self.0 < 8);
        self.0 as u64
    }
}

#[derive(Debug)]
struct ComboOperand(u8);
impl ComboOperand {
    fn value(&self, computer: &Computer) -> u64 {
        match self.0 {
            0 => 0u64,
            1 => 1u64,
            2 => 2u64,
            3 => 3u64,
            4 => computer.register_a,
            5 => computer.register_b,
            6 => computer.register_c,
            _ => panic!("Invalid operand"),
        }
    }
}

impl Computer {
    fn next_instruction(&self) -> Option<Instruction> {
        if self.instruction_pointer >= self.program.len() {
            return None;
        }

        let instruction = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];
        match instruction {
            0 => Some(Instruction::Adv(ComboOperand(operand))),
            1 => Some(Instruction::Bxl(LiteralOperand(operand))),
            2 => Some(Instruction::Bst(ComboOperand(operand))),
            3 => Some(Instruction::Jnz(LiteralOperand(operand))),
            4 => Some(Instruction::Bxc),
            5 => Some(Instruction::Out(ComboOperand(operand))),
            6 => Some(Instruction::Bdv(ComboOperand(operand))),
            7 => Some(Instruction::Cdv(ComboOperand(operand))),
            _ => panic!("Invalid instruction"),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Adv(operand) => {
                let value = operand.value(self);
                self.register_a = self.register_a >> value;
                self.instruction_pointer += 2;
            }
            Instruction::Bxl(operand) => {
                let value = operand.value();
                self.register_b = self.register_b ^ value;
                self.instruction_pointer += 2;
            }
            Instruction::Bst(operand) => {
                let value = operand.value(self);
                self.register_b = value & 0b111;
                self.instruction_pointer += 2;
            }
            Instruction::Jnz(operand) => {
                if self.register_a != 0 {
                    let value = operand.value();

                    assert!(value as usize != self.instruction_pointer);
                    self.instruction_pointer = value as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            Instruction::Bxc => {
                self.register_b = self.register_b ^ self.register_c;
                self.instruction_pointer += 2;
            }
            Instruction::Out(operand) => {
                let value = operand.value(self);
                self.output.push((value & 0b111) as u8);
                self.instruction_pointer += 2;
            }
            Instruction::Bdv(operand) => {
                let value = operand.value(self);
                self.register_b = self.register_a >> value;
                self.instruction_pointer += 2;
            }
            Instruction::Cdv(operand) => {
                let value = operand.value(self);
                self.register_c = self.register_a >> value;
                self.instruction_pointer += 2;
            }
        }
    }

    fn run(mut self) -> Self {
        while let Some(instruction) = self.next_instruction() {
            self.execute_instruction(instruction);
        }

        self
    }

    fn print_output(&self) -> String {
        self.output
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn main() -> Result<()> {
    let input = parse_input("input.txt")?;
    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("Part 1: {}", part_one_answer);
    println!("Part 2: {}", part_two_answer);

    Ok(())
}

fn parse_input(filename: &str) -> Result<Input> {
    let reader = BufReader::new(File::open(filename)?);
    let mut lines = reader.lines();

    let register_a_str = lines.next().unwrap()?;
    let register_b_str = lines.next().unwrap()?;
    let register_c_str = lines.next().unwrap()?;
    lines.next().unwrap()?;
    let program_str = lines.next().unwrap()?;

    assert!(lines.next().is_none());

    let register_a = register_a_str[12..].parse::<u64>()?;
    let register_b = register_b_str[12..].parse::<u64>()?;
    let register_c = register_c_str[12..].parse::<u64>()?;
    let program = program_str[9..]
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    Ok(Computer {
        register_a,
        register_b,
        register_c,
        instruction_pointer: 0,
        program,
        output: Vec::new(),
    })
}

fn part_one(input: &Input) -> String {
    input.clone().run().print_output()
}

fn part_two(input: &Input) -> u64 {
    find_next_suffix(input, 0).unwrap()
}

fn output_with_register_a(computer: &Computer, register_a: u64) -> Vec<u8> {
    let mut computer = computer.clone();
    computer.register_a = register_a;
    computer.run().output
}

fn find_next_suffix(computer: &Computer, register_a_prefix: u64) -> Option<u64> {
    (0..8)
        .filter_map(|suffix| {
            let register_a = register_a_prefix << 3 | suffix;
            let output = output_with_register_a(computer, register_a);

            if output.len() > computer.program.len() {
                return None;
            }

            if output.len() == computer.program.len() {
                if output == computer.program {
                    return Some(register_a);
                }

                return None;
            }

            let suffix_index = computer.program.len() - output.len();

            if output != computer.program[suffix_index..] {
                return None;
            }

            find_next_suffix(computer, register_a)
        })
        .take(1)
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv() -> Result<()> {
        let computer = Computer {
            register_a: 0b00111,
            register_b: 0b01111,
            register_c: 0x10111,
            instruction_pointer: 0,
            program: vec![0, 1],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_a, 0b0011);

        Ok(())
    }

    #[test]
    fn test_bxl() -> Result<()> {
        let computer = Computer {
            register_a: 0,
            register_b: 5,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![1, 2],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_b, 7);
        let computer = Computer {
            register_a: 0,
            register_b: 1,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![1, 1],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_b, 0);

        Ok(())
    }

    #[test]
    fn test_bst() -> Result<()> {
        let computer = Computer {
            register_a: 0b0101010101,
            register_b: 0,
            register_c: 9,
            instruction_pointer: 0,
            program: vec![2, 4],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_b, 0b101);

        Ok(())
    }

    #[test]
    fn test_jnz() -> Result<()> {
        let computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![3, 4, 2, 1],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_b, 1);

        let computer = Computer {
            register_a: 1,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![3, 4, 2, 1],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_b, 0);

        Ok(())
    }

    #[test]
    fn test_bxc() -> Result<()> {
        let computer = Computer {
            register_a: 0,
            register_b: 0b101,
            register_c: 0b110,
            instruction_pointer: 0,
            program: vec![4, 0],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_b, 0b011);

        Ok(())
    }

    #[test]
    fn test_out() -> Result<()> {
        let computer = Computer {
            register_a: 1,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![5, 4],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.output, vec![1]);

        Ok(())
    }

    #[test]
    fn test_bdv() -> Result<()> {
        let computer = Computer {
            register_a: 0b00111,
            register_b: 0b01111,
            register_c: 0x10111,
            instruction_pointer: 0,
            program: vec![6, 1],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_b, 0b0011);

        Ok(())
    }

    #[test]
    fn test_cdv() -> Result<()> {
        let computer = Computer {
            register_a: 0b00111,
            register_b: 0b01111,
            register_c: 0x10111,
            instruction_pointer: 0,
            program: vec![7, 1],
            output: Vec::new(),
        }
        .run();
        assert_eq!(computer.register_c, 0b0011);

        Ok(())
    }

    #[test]
    fn example_1() -> Result<()> {
        let mut computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            instruction_pointer: 0,
            program: vec![2, 6],
            output: Vec::new(),
        };

        let instruction = computer.next_instruction().unwrap();
        computer.execute_instruction(instruction);

        assert_eq!(computer.register_b, 1);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let computer = Computer {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            output: Vec::new(),
        };

        let output = computer.run().print_output();
        assert_eq!(output, "0,1,2");
        Ok(())
    }

    #[test]
    fn example_3() -> Result<()> {
        let computer = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            output: Vec::new(),
        }
        .run();

        let output = computer.print_output();

        assert_eq!(computer.register_a, 0);
        assert_eq!(output, "4,2,5,6,7,7,7,7,3,1,0");
        Ok(())
    }

    #[test]
    fn example_4() -> Result<()> {
        let computer = Computer {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![1, 7],
            output: Vec::new(),
        }
        .run();

        assert_eq!(computer.register_b, 26);
        Ok(())
    }

    #[test]
    fn example_5() -> Result<()> {
        let computer = Computer {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            instruction_pointer: 0,
            program: vec![4, 0],
            output: Vec::new(),
        }
        .run();

        assert_eq!(computer.register_b, 44354);
        Ok(())
    }

    #[test]
    fn example_main() -> Result<()> {
        let output = parse_input("example.txt")?.run().print_output();

        assert_eq!(&output, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn example_suffix() -> Result<()> {
        let computer = parse_input("example.txt")?;
        let suffix = find_next_suffix(&computer, 0);

        assert_eq!(suffix, Some(117440));
        Ok(())
    }
}
