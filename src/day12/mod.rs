use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;

pub trait Mover {
    fn step(&mut self, m: Move) -> &Location;
}

pub struct BoatByWaypoint {
    pub location: Location,
    waypoint: Location
}

impl BoatByWaypoint {
    pub fn default() -> Self {
        Self {
            location: Location::new(0, 0, 0),
            waypoint: Location::new(10, 1, 0)
        }
    }
}

impl Mover for BoatByWaypoint {
    fn step(&mut self, m: Move) -> &Location {
        let delta = m.delta_with_anchor(&self.waypoint, &self.location);
        if let Move::FORWARD(_) = m {
            self.location = self.location + delta;
        }
        self.waypoint = self.waypoint + delta;
        &self.location
    }
}

pub struct BoatByItself {
    pub location: Location
}

impl BoatByItself {
    pub fn default() -> Self {
        Self {
            location: Location::new(0, 0, 90)
        }
    }
}

impl Mover for BoatByItself {
    fn step(&mut self, m: Move) -> &Location {
        self.location = self.location + m.delta(&self.location);
        &self.location
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Location {
    x: isize,
    y: isize,
    bearing: isize
}

impl Location {
    fn new(x: isize, y: isize, bearing: isize) -> Self {
        Self { x, y, bearing: bearing % 360 }
    }

    fn manhattan_distance_from(&self, other: &Location) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn manhattan_distance_from_origin(&self) -> isize {
        self.manhattan_distance_from(&Location::new(0, 0, 0))
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

#[derive(PartialEq, Eq)]
pub enum Move {
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

    fn delta_with_anchor(&self, other: &Location, anchor: &Location) -> Location {
        match self {
            Move::LEFT(amount) => {
                let rotation = (*amount as f64).to_radians();
                let zero_x = other.x - anchor.x;
                let zero_y = other.y - anchor.y;
                let x = zero_x * rotation.cos() as isize - zero_y * rotation.sin() as isize;
                let y = zero_x * rotation.sin() as isize + zero_y * rotation.cos() as isize;
                Location::new(x - other.x + anchor.x, y - other.y + anchor.y, 0)
            },
            Move::RIGHT(amount) => {
                let rotation = (*amount as f64).to_radians();
                let zero_x = other.x - anchor.x;
                let zero_y = other.y - anchor.y;
                let x = zero_x * rotation.cos() as isize + zero_y * rotation.sin() as isize;
                let y = -zero_x * rotation.sin() as isize + zero_y * rotation.cos() as isize;
                Location::new(x - other.x + anchor.x, y - other.y + anchor.y, 0)
            },
            Move::FORWARD(amount) => Location::new((other.x - anchor.x) * amount, (other.y - anchor.y)  * amount, 0),
            _ => self.delta(other)
        }
    }
}

pub fn solve(input: &File, mover: &mut impl Mover) {
    let moves = parse_input(input);
    for m in moves {
        mover.step(m);
    }
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
