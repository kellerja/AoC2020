use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

pub enum InputVersion {
    V1, V2
}

enum Input {
    ValueMask(ValueBitMask),
    AddressMask(AddressBitMask),
    Command(AssignMemory)
}

#[derive(Debug)]
struct AssignMemory(u64, u64);

impl AssignMemory {
    fn new(address: &str, value: &str) -> Self {
        Self(address.parse().unwrap(), value.parse().unwrap())
    }
}

trait BitMask {
    fn apply(&self, memory: &mut HashMap<u64, u64>, value: &AssignMemory);
}

struct ValueBitMask {
    and_mask: u64,
    or_mask: u64
}

impl ValueBitMask {
    fn new(mask: &str) -> Self {
        let and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
        let or_mask = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        Self { and_mask, or_mask }
    }
}

impl BitMask for ValueBitMask {
    fn apply(&self, memory: &mut HashMap<u64, u64>, value: &AssignMemory) {
        memory.insert(value.0, (value.1 & self.and_mask) | self.or_mask);
    }
}

struct AddressBitMask {
    one_setter: u64,
    floating_masks: Vec<(u64, u64)>
}

impl AddressBitMask {
    fn new(mask: &str) -> Self {
        let one_setter = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        let and_mask_template = mask.replace("0", "1");
        let or_mask_template = mask.replace("1", "0");
        let x_count = mask.matches("X").count();
        let x_positions: Vec<usize> = mask.chars().positions(|c| c == 'X').collect();
        let floating_masks = (0..(2 as u64).pow(x_count as u32))
            .map(|truth_table_input| {
                let mut and_mask: Vec<char> = and_mask_template.chars().collect();
                let mut or_mask: Vec<char> = or_mask_template.chars().collect();
                for (i, &x_pos) in x_positions.iter().enumerate() {
                    let bit = bit_at_index(truth_table_input, i as u8);
                    and_mask[x_pos] = bit;
                    or_mask[x_pos] = bit;
                }
                (and_mask.iter().collect::<String>(), or_mask.iter().collect::<String>())
            }).map(|(and_mask, or_mask)|
                (u64::from_str_radix(&and_mask, 2).unwrap(), u64::from_str_radix(&or_mask, 2).unwrap())).collect();
        Self { floating_masks, one_setter }
    }
}

impl BitMask for AddressBitMask {
    fn apply(&self, memory: &mut HashMap<u64, u64>, value: &AssignMemory) {
        let address = self.one_setter | value.0;
        for address in self.floating_masks.iter().map(|(and_mask, or_mask)| (address | or_mask) & and_mask) {
            memory.insert(address, value.1);
        }
    }
}

fn bit_at_index(num: u64, idx: u8) -> char {
    let bit = (num & (1 << idx)) >> idx;
    if bit == 0 { '0' } else { '1' }
}

pub fn solve(input: &File, version: InputVersion) -> Option<u64> {
    let input = parse_input(input, version);
    let mut memory = HashMap::new();
    let mut mask: Box<dyn BitMask> = Box::from(ValueBitMask::new(&"X".repeat(36)));
    for command in input {
        match command {
            Input::ValueMask(new_mask) => mask = Box::from(new_mask),
            Input::AddressMask(new_mask) => mask = Box::from(new_mask),
            Input::Command(cmd) => mask.apply(&mut memory, &cmd)
        }
    }
    Some(memory.values().sum())
}

fn parse_input(input: &File, version: InputVersion) -> Vec<Input> {
    let mask_pattern = Regex::new(r"mask = (?P<mask>[X10]+)").unwrap();
    let set_memory_pattern = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        if let Some(cap) = mask_pattern.captures(&line) {
            let mask = &cap["mask"];
            match version {
                InputVersion::V1 => Input::ValueMask(ValueBitMask::new(mask)),
                InputVersion::V2 => Input::AddressMask(AddressBitMask::new(mask))
            }
        } else if let Some(cap) = set_memory_pattern.captures(&line) {
            Input::Command(AssignMemory::new(&cap["address"], &cap["value"]))
        } else {
            panic!("Could not parse line: {}", line);
        }
    }).collect()
}
