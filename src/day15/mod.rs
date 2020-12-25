use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn solve(input: &File, nth: usize) -> Option<usize> {
    let input = parse_input(input);
    if input.is_empty() {
        return None;
    }
    let mut last_spoken_index = vec![None; nth];
    for (i, &num) in input[..input.len() - 1].iter().enumerate() {
        last_spoken_index[num] = Some(i + 1);
    }
    let mut previous = *input.last().unwrap();
    for i in input.len()..nth {
        let next = if let Some(idx) = last_spoken_index.get(previous).unwrap() {
            i - idx
        } else { 0 };
        last_spoken_index[previous] = Some(i);
        previous = next;
    }
    Some(previous)
}

fn parse_input(input: &File) -> Vec<usize> {
    let mut lines = BufReader::new(input).lines();
    lines.next().unwrap().unwrap().split(",")
        .map(|s| s.to_owned().parse().unwrap()).collect()
}
