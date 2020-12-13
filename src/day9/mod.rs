use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn find_pair_with_sum(array: &[usize], sum: usize) -> Option<(usize, usize)> {
    for (a_idx, a) in array.iter().enumerate() {
        for (b_idx, b) in array[a_idx + 1..].iter().enumerate() {
            if a + b == sum {
                return Some((a_idx, b_idx))
            }
        }
    }
    None
}

fn find_number_without_preable_sum(numbers: &[usize], preamble_size: usize) -> Option<usize> {
    let preamble_iterator = numbers.windows(preamble_size);
    for (i, preamble) in preamble_iterator.enumerate() {
        let target = numbers[i + preamble_size];
        if find_pair_with_sum(preamble, target).is_none() {
            return Some(target)
        }
    }
    None
}

pub fn solve(input: &File) -> Option<usize> {
    const PREAMBLE_SIZE: usize = 25;
    let numbers = parse_input(input);
    find_number_without_preable_sum(&numbers, PREAMBLE_SIZE)
}

fn parse_input(input: &File) -> Vec<usize> {
    BufReader::new(input).lines().map(|line| *&line.unwrap().parse().unwrap()).collect()
}
