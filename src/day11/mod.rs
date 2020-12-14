use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct CloseNeighbourConfiguration;
pub struct VisibleNeighbourConfiguration;

pub trait SimulationConfiguration {
    fn get_visibility_limit(&self) -> Option<usize> {
        None
    }

    fn get_acceptable_neighbours_count(&self) -> usize {
        0
    }
}

impl SimulationConfiguration for CloseNeighbourConfiguration {
    fn get_visibility_limit(&self) -> Option<usize> {
        Some(1)
    }

    fn get_acceptable_neighbours_count(&self) -> usize {
        3
    }
}

impl SimulationConfiguration for VisibleNeighbourConfiguration {
    fn get_acceptable_neighbours_count(&self) -> usize {
        4
    }
}

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';
const FLOOR: char = '.';

fn get_neighbours(map: &Vec<Vec<char>>, location: (usize, usize), step_limit: Option<usize>) -> Vec<char> {
    let mut neighbours = Vec::with_capacity(8);
    let bounds =  (0, map.len() as isize, 0, map.get(location.0).unwrap().len() as isize);
    let step_limit = step_limit.unwrap_or(map.len());
    for (row_offset, col_offset) in &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        let mut neighbour_row = location.0 as isize;
        let mut neighbour_col = location.1 as isize;
        let mut step = 0;
        loop {
            neighbour_row += row_offset;
            neighbour_col += col_offset;
            if step >= step_limit || neighbour_row < bounds.0 || neighbour_row >= bounds.1
                || neighbour_col < bounds.2 || neighbour_col >= bounds.3 {
                    break
            } else if let Some(tile) = map.get(neighbour_row as usize).and_then(|row| row.get(neighbour_col as usize)) {
                if tile != &FLOOR {
                    neighbours.push(tile.to_owned());
                    break;
                }
            }
            step += 1;
        }
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

fn step_simulation(map: &Vec<Vec<char>>, config: &impl SimulationConfiguration) -> Vec<Vec<char>> {
    let mut new_map = Vec::with_capacity(map.len());
    for (row_idx, row) in map.iter().enumerate() {
        let mut new_row = Vec::with_capacity(row.len());
        for (col_idx, seat) in row.iter().enumerate() {
            let (_empty, taken) = count_occupation(&get_neighbours(map, (row_idx, col_idx), config.get_visibility_limit()));
            if seat == &EMPTY_SEAT && taken == 0 {
                new_row.push(OCCUPIED_SEAT);
            } else if seat == &OCCUPIED_SEAT && taken > config.get_acceptable_neighbours_count() {
                new_row.push(EMPTY_SEAT);
            } else {
                new_row.push(seat.to_owned());
            }
        }
        new_map.push(new_row);
    }
    new_map
}

pub fn solve(input: &File, config: &impl SimulationConfiguration) -> Option<usize> {
    let mut seating_plan = parse_input(input);
    loop {
        let new_plan = step_simulation(&seating_plan, config);
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
