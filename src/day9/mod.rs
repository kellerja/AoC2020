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

fn find_consecutive_slice_with_sum(array: &[usize], target_sum: usize) -> Option<(usize, usize)> {
    let mut start_idx = 0;
    let mut end_idx = 1;
    let mut sum = array[start_idx..end_idx + 1].iter().sum::<usize>() as isize;
    loop {
        if sum == target_sum as isize {
            return Some((start_idx, end_idx));
        } else if sum > target_sum as isize {
            sum -= array[start_idx] as isize;
            start_idx += 1;
        } else {
            end_idx += 1;
            sum += array[end_idx] as isize;
        }
        if start_idx == array.len() || end_idx + 1 > array.len() {
            return None;
        }
    }
}

fn find_number_without_preamble_sum(numbers: &[usize], preamble_size: usize) -> Option<usize> {
    let preamble_iterator = numbers.windows(preamble_size);
    for (i, preamble) in preamble_iterator.enumerate() {
        let target = numbers[i + preamble_size];
        if find_pair_with_sum(preamble, target).is_none() {
            return Some(target)
        }
    }
    None
}

pub fn solve(input: &File, preamble_size: usize) -> (Option<usize>, Option<usize>) {
    let numbers = parse_input(input);
    let target = find_number_without_preamble_sum(&numbers, preamble_size);
    let mut target_sum_addends_sum = None;
    if let Some(target) = target {
        target_sum_addends_sum = find_consecutive_slice_with_sum(&numbers, target).and_then(|(start_idx, end_idx)| {
            let slice = &numbers[start_idx..end_idx + 1];
            Some(slice.iter().min().unwrap() + slice.iter().max().unwrap())
        });
    }
    (target, target_sum_addends_sum)
}

fn parse_input(input: &File) -> Vec<usize> {
    BufReader::new(input).lines().map(|line| *&line.unwrap().parse().unwrap()).collect()
}
