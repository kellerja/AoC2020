use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn solve(input: &File) -> Option<isize> {
    let input = parse_input(input);
    None
}

fn parse_input(input: &File) -> Vec<String> {
    BufReader::new(input).lines().map(|line| line.unwrap()).collect()
}
