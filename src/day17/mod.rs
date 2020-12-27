use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};
use radix_fmt::radix;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Coordinate {
    coordinates: Vec<i64>
}

impl Coordinate {
    fn delta(&self, delta: &[i64]) -> Coordinate {
        Coordinate {
            coordinates: self.coordinates.iter().zip(delta.iter()).map(|(c, dc)| c + dc).collect()
        }
    }

    fn neighbours(&self) -> Vec<Coordinate> {
        const CHOICES: [i64; 3] = [-1, 0, 1];
        let dimensions = self.coordinates.len() as u8;
        let deltas_count = CHOICES.len().pow(dimensions as u32);
        let mut neighbours = Vec::with_capacity((deltas_count - 1) as usize);
        for i in 0..deltas_count {
            let choices_index = format!("{}", radix(i, CHOICES.len() as u8));
            let choices_index = "0".repeat(dimensions as usize - choices_index.len()) + &choices_index;
            let delta: Vec<i64> = choices_index.chars()
                .map(|idx| CHOICES[idx.to_digit(10).unwrap() as usize]).collect();
            if delta == vec![0; delta.len()] {
                continue;
            }
            neighbours.push(self.delta(&delta));
        }
        neighbours
    }
}

#[derive(Clone)]
struct ConwayCubesState {
    active_cubes: HashSet<Coordinate>
}

impl IntoIterator for ConwayCubesState {
    type Item = ConwayCubesState;
    type IntoIter = Cycles;

    fn into_iter(self) -> Cycles {
        Cycles { current_state: self }
    }
}

struct Cycles {
    current_state: ConwayCubesState
}

impl Iterator for Cycles {
    type Item = ConwayCubesState;

    fn next(&mut self) -> Option<Self::Item> {
        let active_cubes = &self.current_state.active_cubes;
        let mut active_neighbour_count: HashMap<Coordinate, u64> = HashMap::new();
        for cube in active_cubes {
            for neighbour in cube.neighbours() {
            let neighbour_count = active_neighbour_count.entry(neighbour).or_insert(0);
            *neighbour_count += 1;
            }
        }
        let mut new_active_cubes = HashSet::new();
        for (cube, active_neighbours_count) in active_neighbour_count {
            let active = active_cubes.contains(&cube);
            if active && (2..=3).contains(&active_neighbours_count) {
                new_active_cubes.insert(cube);
            } else if !active && active_neighbours_count == 3 {
                new_active_cubes.insert(cube);
            }
        }
        let new_state = ConwayCubesState {active_cubes : new_active_cubes};
        self.current_state = new_state.clone();
        Some(new_state)
    }
}

pub fn solve(input: &File, default_extra_dimensions: &[i64]) -> Option<usize> {
    let mut initial_state = parse_input(input, default_extra_dimensions).into_iter();
    for _ in 0..5 {
        initial_state.next();
    }
    let final_state = initial_state.next().unwrap();
    Some(final_state.active_cubes.len())
}

fn parse_input(input: &File, default_extra_dimensions: &[i64]) -> ConwayCubesState {
    let lines = BufReader::new(input).lines();
    let mut active_cubes = HashSet::new();
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        for (x, cube_state) in line.chars().enumerate() {
            if cube_state == '#' {
                let mut coordinates = vec![x as i64, y as i64];
                coordinates.extend(default_extra_dimensions);
                active_cubes.insert(Coordinate { coordinates });
            }
        }
    }
    ConwayCubesState {
        active_cubes
    }
}
