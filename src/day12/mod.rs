use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Location {
    x: isize,
    y: isize,
    bearing: isize
}

impl Location {
    fn new(x: isize, y: isize, bearing: isize) -> Location {
        Location { x, y, bearing: bearing % 360 }
    }

    fn default() -> Location {
        Location::new(0, 0, 90)
    }

    fn manhattan_distance_from(&self, other: &Location) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Location {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            bearing: (self.bearing + other.bearing) % 360
        }
    }
}

enum Move {
    NORTH(isize),
    SOUTH(isize),
    EAST(isize),
    WEST(isize),
    LEFT(isize),
    RIGHT(isize),
    FORWARD(isize)
}

impl Move {
    fn delta(&self, other: &Location) -> Location {
        match self {
            Move::NORTH(amount) => Location::new(0, *amount, 0),
            Move::SOUTH(amount) => Location::new(0, -amount, 0),
            Move::EAST(amount) => Location::new(*amount, 0, 0),
            Move::WEST(amount) => Location::new(-amount, 0, 0),
            Move::LEFT(amount) => Location::new(0, 0, -amount),
            Move::RIGHT(amount) => Location::new(0, 0, *amount),
            Move::FORWARD(amount) => {
                let bearing = (other.bearing as f64).to_radians();
                Location::new(amount * bearing.sin() as isize, amount * bearing.cos() as isize, 0)
            }
        }
    }
}

pub fn solve(input: &File) -> Option<isize> {
    let moves = parse_input(input);
    let mut boat = Location::default();
    for m in moves {
        let d = m.delta(&boat);
        boat = boat + d;
    }
    Some(boat.manhattan_distance_from(&Location::default()))
}

fn parse_input(input: &File) -> Vec<Move> {
    BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        let (cmd, amount) = line.split_at(1);
        let amount = amount.parse().unwrap();
        match cmd {
            "N" => Move::NORTH(amount),
            "S" => Move::SOUTH(amount),
            "E" => Move::EAST(amount),
            "W" => Move::WEST(amount),
            "L" => Move::LEFT(amount),
            "R" => Move::RIGHT(amount),
            "F" => Move::FORWARD(amount),
            _ => panic!("Unknown command {}", cmd)
        }
    }).collect()
}
