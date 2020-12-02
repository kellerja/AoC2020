use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

trait PasswordPolicy {
    fn matched(&self, password: &str) -> bool;
}

struct CharCountPolicy {
    letter: char,
    min_count: usize,
    max_count: usize,
}

impl CharCountPolicy {
    fn parse(input: &str) -> CharCountPolicy {
        let chat_policy_pattern = Regex::new(r"(?P<min_count>[[:digit:]]+)-(?P<max_count>[[:digit:]]+) (?P<char>[[:alpha:]])").unwrap();
        let cap = chat_policy_pattern.captures(input).unwrap();
        CharCountPolicy {
            letter: cap["char"].chars().next().unwrap(),
            min_count: cap["min_count"].parse::<usize>().unwrap(),
            max_count: cap["max_count"].parse::<usize>().unwrap()
        }
    }
}

impl PasswordPolicy for CharCountPolicy {
    fn matched(&self, password: &str) -> bool {
        let mut match_count = 0;
        for c in password.chars() {
            if c == self.letter {
                match_count += 1;
            }
        }
        return match_count >= self.min_count && match_count <= self.max_count;
    }
}

pub fn solve(input: &File) -> Option<usize> {
    let mut correct_count = 0;
    let parsed_input = parse_input(input);
    if parsed_input.is_empty() {
        return None
    }
    for line in parsed_input {
        if line.policy.matched(&line.password) {
            correct_count += 1
        }
    }
    Some(correct_count)
}

struct ParsedLine { password: String, policy: Box<dyn PasswordPolicy> }

fn parse_input(input: &File) -> Vec<ParsedLine> {
    io::BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        let mut parsed = line.split(":");
        let policy = parsed.next().unwrap().trim();
        let password = parsed.next().unwrap().trim();
        ParsedLine {
            password: password.to_owned(),
            policy: Box::new(CharCountPolicy::parse(policy))
        }
    }).collect()
}
