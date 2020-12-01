use std::fs::File;
use std::io::{self, BufRead};

pub fn solve(input: &File, sum_varible_count: u8) -> Option<i32> {
    let numbers = parse_input(input);
    find_product_with_target_sum(&numbers, 0, 2020, sum_varible_count)
}

fn find_product_with_target_sum(input: &Vec<i32>, sum: i32, sum_target: i32, sum_varible_count: u8) -> Option<i32> {
    if sum_varible_count <= 0 {
        if sum == sum_target {
            return Some(1)
        }
        return None
    }
    for a in input {
        if let Some(value) = find_product_with_target_sum(input, sum + a, sum_target, sum_varible_count - 1) {
            return Some(a * value)
        }
    }
    None
}

fn parse_input(input: &File) -> Vec<i32> {
    io::BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        line.parse::<i32>().expect(&format!("Expected number, but got: {}", line))
    }).collect()
}
