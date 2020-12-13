use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';
const _FLOOR: char = '.';

fn get_neighbours(map: &Vec<Vec<char>>, target: (usize, usize)) -> Vec<char> {
    let mut neighbours = Vec::with_capacity(8);
    // top, bottom, left, right
    let bounds = (0, map.len() as isize, 0, map.get(target.0).unwrap().len() as isize);
    for (row_offset, col_offset) in &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        let neighbour_row = target.0 as isize + row_offset;
        if neighbour_row < bounds.0 || neighbour_row >= bounds.1 {
            continue
        }
        let neighbour_col = target.1 as isize + col_offset;
        if neighbour_col < bounds.2 || neighbour_col >= bounds.3 {
            continue
        }
        neighbours.push(map.get(neighbour_row as usize).unwrap().get(neighbour_col as usize).unwrap().to_owned());
    }
    neighbours
}

fn count_occupation(tiles: &[char]) -> (usize, usize) {
    let mut occupation = (0, 0);
    for tile in tiles {
        match tile {
            &EMPTY_SEAT => occupation.0 += 1,
            &OCCUPIED_SEAT => occupation.1 += 1,
            _ => continue
        }
    }
    occupation
}

fn step_simulation(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::with_capacity(map.len());
    for (row_idx, row) in map.iter().enumerate() {
        let mut new_row = Vec::with_capacity(row.len());
        for (col_idx, seat) in row.iter().enumerate() {
            let (_empty, taken) = count_occupation(&get_neighbours(map, (row_idx, col_idx)));
            if seat == &EMPTY_SEAT && taken == 0 {
                new_row.push(OCCUPIED_SEAT);
            } else if seat == &OCCUPIED_SEAT && taken >= 4 {
                new_row.push(EMPTY_SEAT);
            } else {
                new_row.push(seat.to_owned());
            }
        }
        new_map.push(new_row);
    }
    new_map
}

pub fn solve(input: &File) -> Option<usize> {
    let mut seating_plan = parse_input(input);
    loop {
        let new_plan = step_simulation(&seating_plan);
        let is_same_plan = seating_plan == new_plan;
        seating_plan = new_plan;
        if is_same_plan {
            break
        }
    }
    Some(seating_plan.iter().fold(0, |acc, current| {
        let (_empty, occupied) = count_occupation(current);
        acc + occupied
    }))
}

fn parse_input(input: &File) -> Vec<Vec<char>> {
    BufReader::new(input).lines().map(|line| line.unwrap().chars().collect()).collect()
}
