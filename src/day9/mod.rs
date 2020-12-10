use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn find_preamble_sum(numbers: &[usize], preamble_size: usize, target_index: usize) -> Option<(usize, usize)> {
    let number = *numbers.get(target_index).unwrap();

    let preamble_start = target_index - preamble_size;
    let preamble_end = target_index;
    for start_i in preamble_start..preamble_end - 1 {
        let a = numbers.get(start_i).unwrap();
        for end_i in start_i + 1..preamble_end {
            let b = numbers.get(end_i).unwrap();
            if a + b == number {
                return Some((*a, *b))
            }
        }
    }
    None
}

pub fn solve(input: &File) -> Option<usize> {
    const PREAMBLE_SIZE: usize = 25;
    let numbers = parse_input(input);
    for i in PREAMBLE_SIZE..numbers.len() {
        if find_preamble_sum(&numbers, PREAMBLE_SIZE, i).is_none() {
            return Some(*numbers.get(i).unwrap())
        }
    }
    None
}

fn parse_input(input: &File) -> Vec<usize> {
    BufReader::new(input).lines().map(|line| *&line.unwrap().parse().unwrap()).collect()
}
