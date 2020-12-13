use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn solve(input: &File) -> Option<usize> {
    None
}

fn parse_input(input: &File) -> Vec<usize> {
    BufReader::new(input).lines().map(|line| *&line.unwrap().parse().unwrap()).collect()
}