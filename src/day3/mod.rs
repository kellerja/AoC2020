use std::fs::File;
use std::io::{self, BufRead};

pub const PART_1_SLOPES: [Slope; 1] = [Slope { right: 3, down: 1 }];
pub const PART_2_SLOPES: [Slope; 5] = [
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 }
];

#[derive(Clone)]
pub struct Slope {
    right: usize,
    down: usize
}

pub fn solve(input: &File, slopes: &Vec<Slope>) -> Option<usize> {
    if slopes.is_empty() {
        return None
    }
    let matrix = parse_input(input);
    if matrix.is_empty() {
        return None
    }
    
    let mut total_trees = 1;
    for slope in slopes {
        total_trees *= count_trees(&matrix, slope);
    }
    Some(total_trees)
}

const TREE: char = '#';

fn count_trees(matrix: &Vec<Vec<char>>, slope: &Slope) -> usize {
    let mut tree_count = 0;
    for (i, column) in matrix[slope.down..].iter().step_by(slope.down).enumerate() {
        let col_index = ((i + 1) * slope.right) % column.len();
        if column[col_index] == TREE {
            tree_count += 1;
        }
    }
    tree_count
}

fn parse_input(input: &File) -> Vec<Vec<char>> {
    io::BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        line.chars().collect()
    }).collect()
}
