use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use regex::Regex;

pub fn solve(input: &File) -> Option<usize> {
    Some(parse_input(input).iter().map(|group| group.len()).sum())
}

fn parse_input(input: &File) -> Vec<HashSet<char>> {
    let mut reader = BufReader::new(input);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    let new_entry_pattern = Regex::new(r"(?m)(\r\n|\n){2}").unwrap();
    new_entry_pattern.split(contents.as_str())
        .map(|group| group.chars().filter(|c| !c.is_whitespace()).collect())
        .collect()
}
