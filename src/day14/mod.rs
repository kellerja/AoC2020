use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

enum Input {
    Mask(BitMask),
    Command(AssignMemory)
}

struct AssignMemory(u64, u64);

impl AssignMemory {
    fn new(address: &str, value: &str) -> AssignMemory {
        AssignMemory(address.parse().unwrap(), value.parse().unwrap())
    }
}

struct BitMask {
    and_mask: u64,
    or_mask: u64
}

impl BitMask {
    fn new(mask: &str) -> BitMask {
        let and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
        let or_mask = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        BitMask { and_mask, or_mask }
    }

    fn apply(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }
}

pub fn solve(input: &File) -> Option<u64> {
    let input = parse_input(input);
    let mut memory = HashMap::new();
    let mut mask = BitMask::new(&"X".repeat(36));
    for command in input {
        match command {
            Input::Mask(new_mask) => mask = new_mask,
            Input::Command(cmd) => { memory.insert(cmd.0, mask.apply(cmd.1)); }
        }
    }
    Some(memory.values().sum())
}

fn parse_input(input: &File) -> Vec<Input> {
    let mask_pattern = Regex::new(r"mask = (?P<mask>[X10]+)").unwrap();
    let set_memory_pattern = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        if let Some(cap) = mask_pattern.captures(&line) {
            Input::Mask(BitMask::new(&cap["mask"]))
        } else if let Some(cap) = set_memory_pattern.captures(&line) {
            Input::Command(AssignMemory::new(&cap["address"], &cap["value"]))
        } else {
            panic!("Could not parse line: {}", line);
        }
    }).collect()
}
