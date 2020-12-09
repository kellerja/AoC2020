use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Instruction {
    NOP,
    ACC(isize),
    JMP(isize),
    END
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let mut parts = input.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "nop" => Instruction::NOP,
            "acc" => {
                let amount = parts.next().unwrap().replace("+", "").parse().unwrap();
                Instruction::ACC(amount)
            },
            "jmp" => {
                let amount = parts.next().unwrap().replace("+", "").parse().unwrap();
                Instruction::JMP(amount)
            },
            _ => panic!("Unknown instruction {}", instruction)
        }
    }
}

struct CPU {
    ic: isize,
    accumulator: isize,
    visited_instruction_count: Vec<usize>
}

impl CPU {

    fn get_instruction(&mut self, program: &Vec<Instruction>) -> Instruction {
        let exec_count = self.visited_instruction_count.get_mut(self.ic as usize).unwrap();
        *exec_count += 1;
        if *exec_count >= 2 {
            Instruction::END
        } else {
            program.get(self.ic as usize).unwrap().to_owned()
        }
    }

    fn execute(program: &Vec<Instruction>) -> CPU {
        let mut cpu = CPU {
            ic: 0,
            accumulator: 0,
            visited_instruction_count: vec![0; program.len()]
        };

        loop {
            if cpu.ic < 0 {
                panic!("Negative IC ({})", cpu.ic)
            }
            let instruction = cpu.get_instruction(&program);
            match instruction {
                Instruction::NOP => {
                    cpu.ic += 1;
                },
                Instruction::ACC(amount) => {
                    cpu.accumulator += amount;
                    cpu.ic += 1;
                },
                Instruction::JMP(amount) => {
                    cpu.ic += amount;
                },
                Instruction::END => {
                    break;
                }
            }
        }
        cpu
    }
}

pub fn solve(input: &File) -> Option<isize> {
    let program = parse_input(input);
    Some(CPU::execute(&program).accumulator)
}

fn parse_input(input: &File) -> Vec<Instruction> {
    BufReader::new(input).lines().map(|line| Instruction::parse(&line.unwrap())).collect()
}
