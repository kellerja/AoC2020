use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub struct CharCountPolicy;
pub struct CharPositionPolicy;

pub struct PolicyData {
    letter: char,
    first_number: usize,
    last_number: usize,
}

pub trait PasswordPolicy {
    fn matches(&self, password: &str, data: &PolicyData) -> bool;
}

impl PolicyData {
    fn parse(input: &str) -> Self {
        let count_policy_pattern = Regex::new(r"(?P<first_number>[[:digit:]]+)-(?P<last_number>[[:digit:]]+) (?P<char>[[:alpha:]])").unwrap();
        let cap = count_policy_pattern.captures(input).unwrap();
        PolicyData {
            letter: cap["char"].chars().next().unwrap(),
            first_number: cap["first_number"].parse::<usize>().unwrap(),
            last_number: cap["last_number"].parse::<usize>().unwrap()
        }
    }
}

impl PasswordPolicy for CharCountPolicy {
    fn matches(&self, password: &str, data: &PolicyData) -> bool {
        let mut match_count = 0;
        for c in password.chars() {
            if c == data.letter {
                match_count += 1;
            }
        }
        return match_count >= data.first_number && match_count <= data.last_number;
    }
}

impl PasswordPolicy for CharPositionPolicy {
    fn matches(&self, password: &str, data: &PolicyData) -> bool {
        let chars: Vec<char> = password.chars().collect();
        let position_a_match = chars[data.first_number - 1] == data.letter;
        let position_b_match = chars[data.last_number - 1] == data.letter;
        return position_a_match ^ position_b_match;
    }
}

pub fn solve(input: &File, policy: impl PasswordPolicy) -> Option<usize> {
    let mut correct_count = 0;
    let parsed_input = parse_input(input);
    if parsed_input.is_empty() {
        return None
    }
    for line in parsed_input {
        if policy.matches(&line.password, &line.data) {
            correct_count += 1;
        }
    }
    Some(correct_count)
}

struct ParsedLine { password: String, data: PolicyData }

fn parse_input(input: &File) -> Vec<ParsedLine> {
    io::BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        let mut parsed = line.split(":");
        let policy = parsed.next().unwrap().trim();
        let password = parsed.next().unwrap().trim();
        ParsedLine {
            password: password.to_owned(),
            data: PolicyData::parse(policy)
        }
    }).collect()
}
