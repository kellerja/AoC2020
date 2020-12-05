use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub enum SearchCriteria {
    HIGHEST,
    EMPTY
}

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

fn find_highest_seat_id(sorted_boarding_passes: &Vec<BoardingPass>) -> Option<usize> {
    sorted_boarding_passes.iter().next().and_then(|pass| Some(pass.seat_id()))
}

fn find_free_seat_id(sorted_boarding_passes: &Vec<BoardingPass>) -> Option<usize> {
    let mut boarding_passes = sorted_boarding_passes.iter();
    let mut previous_pass_seat_id = boarding_passes.next().unwrap().seat_id();
    for pass in boarding_passes {
        let current_pass_seat_id = pass.seat_id();
        if current_pass_seat_id + 1 != previous_pass_seat_id {
            return Some(current_pass_seat_id + 1)
        }
        previous_pass_seat_id = current_pass_seat_id;
    }
    None
}

pub fn solve(input: &File, search: SearchCriteria) -> Option<usize> {
    let mut boarding_passes = parse_input(input);
    if boarding_passes.is_empty() {
        return None;
    }
    boarding_passes.sort_by(|one, other| other.seat_id().cmp(&one.seat_id()));
    match search {
        SearchCriteria::HIGHEST => find_highest_seat_id(&boarding_passes),
        SearchCriteria::EMPTY => find_free_seat_id(&boarding_passes)
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
