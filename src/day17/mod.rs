use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Coordinate(i64, i64, i64);

impl Coordinate {
    fn neighbours(&self) -> Vec<Coordinate> {
        const DELTAS: [(i64, i64, i64); 26] = [
            (-1, -1, -1),  (-1, -1,  0),  (-1, -1, 1),
            (-1,  0, -1),  (-1,  0,  0),  (-1,  0, 1),
            (-1,  1, -1),  (-1,  1,  0),  (-1,  1, 1),
            ( 0, -1, -1),  ( 0, -1,  0),  ( 0, -1, 1),
            ( 0,  0, -1),/*( 0,  0,  0),*/( 0,  0, 1),
            ( 0,  1, -1),  ( 0,  1,  0),  ( 0,  1, 1),
            ( 1, -1, -1),  ( 1, -1,  0),  ( 1, -1, 1),
            ( 1,  0, -1),  ( 1,  0,  0),  ( 1,  0, 1),
            ( 1,  1, -1),  ( 1,  1,  0),  ( 1,  1, 1)
            ];
        DELTAS.iter().map(|(dx, dy, dz)| Coordinate(self.0 + dx, self.1 + dy, self.2 + dz)).collect()
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

pub fn solve(input: &File) -> Option<usize> {
    let mut initial_state = parse_input(input).into_iter();
    for _ in 0..5 {
        initial_state.next();
    }
    let final_state = initial_state.next().unwrap();
    Some(final_state.active_cubes.len())
}

fn parse_input(input: &File) -> ConwayCubesState {
    let z = 1;
    let lines = BufReader::new(input).lines();
    let mut active_cubes = HashSet::new();
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        for (x, cube_state) in line.chars().enumerate() {
            if cube_state == '#' {
                active_cubes.insert(Coordinate(x as i64, y as i64, z));
            }
        }
    }
    ConwayCubesState {
        active_cubes
    }
}
