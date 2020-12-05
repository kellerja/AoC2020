use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct BoardingPass {
    row: usize,
    col: usize
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn search_target(selectors: &str) -> usize {
    let mut lower_index = 0.0;
    let mut higher_index = usize::pow(2, selectors.len() as u32) as f64;
    for row_selector in selectors.chars() {
        if row_selector == 'F' || row_selector == 'L' {
            higher_index = lower_index + f64::floor((higher_index - lower_index) / 2.0)
        } else {
            lower_index = lower_index + f64::ceil((higher_index - lower_index) / 2.0)
        }
    }
    lower_index as usize
}

pub fn solve(input: &File) -> Option<usize> {
    let mut highest_id = -1;
    for pass in parse_input(input) {
        let seat_id = pass.seat_id() as isize;
        if seat_id > highest_id {
            highest_id = seat_id;
        }
    }
    if highest_id < 0 {
        None
    } else {
        Some(highest_id as usize)
    }
}



fn parse_input(input: &File) -> Vec<BoardingPass> {
    BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        let row = search_target(&line[0..7]);
        let col = search_target(&line[7..]);
        BoardingPass { row, col }
    }).collect()
}
