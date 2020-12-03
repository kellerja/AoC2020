use std::fs::File;
use std::io::{self, BufRead};

const TREE: char = '#';

pub fn solve(input: &File) -> Option<i32> {
    let matrix = parse_input(input);
    if matrix.is_empty() {
        return None
    }
    let mut tree_count = 0;
    for (row_index, column) in matrix[1..].iter().enumerate() {
        let col_position = ((row_index + 1) * 3) % column.len();
        if column[col_position] == TREE {
            tree_count += 1;
        }
    }
    Some(tree_count)
}

fn parse_input(input: &File) -> Vec<Vec<char>> {
    io::BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        line.chars().collect()
    }).collect()
}
