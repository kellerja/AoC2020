use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
    END(ExecutionResult)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExecutionResult {
    SUCCESS,
    ERROR
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let mut parts = input.split_whitespace();
        let instruction = parts.next().unwrap();
        let amount = parts.next().unwrap().replace("+", "").parse().unwrap();
        match instruction {
            "nop" => Instruction::NOP(amount),
            "acc" => Instruction::ACC(amount),
            "jmp" => Instruction::JMP(amount),
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

    fn execute(program: &[Instruction]) -> (CPU, ExecutionResult) {
        let mut cpu = CPU {
            ic: 0,
            accumulator: 0,
            visited_instruction_count: vec![0; program.len()]
        };
        let result = cpu.run(program);
        (cpu, result)
    }

    fn get_instruction(&mut self, program: &[Instruction]) -> Instruction {
        let instruction_index = self.ic as usize;
        if instruction_index == program.len() {
            return Instruction::END(ExecutionResult::SUCCESS);
        }
        let exec_count = self.visited_instruction_count.get_mut(instruction_index).unwrap();
        *exec_count += 1;
        if *exec_count >= 2 {
            Instruction::END(ExecutionResult::ERROR)
        } else {
            program.get(instruction_index).unwrap().to_owned()
        }
    }

    fn run(&mut self, program: &[Instruction]) -> ExecutionResult {
        loop {
            if self.ic < 0 {
                panic!("Negative IC ({})", self.ic)
            }
            let instruction = self.get_instruction(&program);
            match instruction {
                Instruction::NOP(_amount) => {
                    self.ic += 1;
                },
                Instruction::ACC(amount) => {
                    self.accumulator += amount;
                    self.ic += 1;
                },
                Instruction::JMP(amount) => {
                    self.ic += amount;
                },
                Instruction::END(result) => {
                    return result
                }
            }
        }
    }
}

fn find_successful_program(broken_program: &[Instruction]) -> Option<Vec<Instruction>> {
    let program = broken_program.to_owned();
    let (_, result) = CPU::execute(&program);
    if result == ExecutionResult::SUCCESS {
        return Some(program);
    }
    for i in 0..program.len() {
        let old_instruction = program.get(i).unwrap();
        let new_instruction = match old_instruction {
            Instruction::NOP(amount) => Instruction::JMP(*amount),
            Instruction::JMP(amount) => Instruction::NOP(*amount),
            _ => continue
        };
        let mut new_program = program.clone();
        new_program[i] = new_instruction;
        let (_, result) = CPU::execute(&new_program);
        if result == ExecutionResult::SUCCESS {
            return Some(new_program);
        }
    }
    None
}

pub fn solve(input: &File, fix_program: bool) -> Option<isize> {
    let program = parse_input(input);
    let runnable_program = if fix_program {
        find_successful_program(&program)
    } else {
        Some(program)
    };
    runnable_program.and_then(|program| Some(CPU::execute(&program).0.accumulator))
}

fn parse_input(input: &File) -> Vec<Instruction> {
    BufReader::new(input).lines().map(|line| Instruction::parse(&line.unwrap())).collect()
}
