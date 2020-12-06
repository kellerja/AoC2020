use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use regex::Regex;

pub struct AnyonesAnswer;
pub struct EveryonesAnswer;

pub trait GroupAnswerParser {
    fn parse(&self, group: &str) -> String;
}

impl GroupAnswerParser for AnyonesAnswer {
    fn parse(&self, group: &str) -> String {
        let unique_answers: HashSet<char> = group.chars().filter(|c| !c.is_whitespace()).collect();
        unique_answers.iter().collect()
    }
}

impl GroupAnswerParser for EveryonesAnswer {
    fn parse(&self, group: &str) -> String {
        let mut unique_answers: Vec<HashSet<char>> = group.split_whitespace()
            .map(|answer| answer.chars().collect())
            .collect();
        let base_set = unique_answers.pop().unwrap();
        base_set.iter().filter(|answer| unique_answers.iter().all(|set| set.contains(answer))).collect()
    }
}

pub fn solve(input: &File, answer_parser: &impl GroupAnswerParser) -> Option<usize> {
    Some(parse_input(input, answer_parser))
}

fn parse_input(input: &File, answer_parser: &impl GroupAnswerParser) -> usize {
    let mut reader = BufReader::new(input);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    let new_entry_pattern = Regex::new(r"(?m)(\r\n|\n){2}").unwrap();
    new_entry_pattern.split(contents.as_str())
        .map(|group| answer_parser.parse(group))
        .map(|group| group.len())
        .sum()
}
