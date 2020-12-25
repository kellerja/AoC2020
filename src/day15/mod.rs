use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn solve(input: &File) -> Option<usize> {
    let input = parse_input(input);
    if input.is_empty() {
        return None;
    }
    let mut latest_index = [None; 2020];
    for (i, &num) in input[..input.len() - 1].iter().enumerate() {
        latest_index[num] = Some(i + 1);
    }
    let mut previous = *input.last().unwrap();
    for i in input.len()..2020 {
        let next = if let Some(idx) = latest_index[previous] {
            i - idx
        } else { 0 };
        latest_index[previous] = Some(i);
        previous = next;
    }
    Some(previous)
}

fn parse_input(input: &File) -> Vec<usize> {
    let mut lines = BufReader::new(input).lines();
    lines.next().unwrap().unwrap().split(",")
        .map(|s| s.to_owned().parse().unwrap()).collect()
}
