use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn solve(input: &File) -> Option<usize> {
    let (earliest_time, schedule) = parse_input(input);
    let initial_timestamps: Vec<usize> = schedule.iter().filter(|time| time != &"x").map(|time| time.parse().unwrap()).collect();
    let mut best_diff = usize::MAX;
    let mut id = None;
    for time in initial_timestamps {
        let diff = ((earliest_time / time + 1) * time).checked_sub(earliest_time).unwrap();
        if diff < best_diff {
            best_diff = diff;
            id = Some(time);
        }
    }
    id.and_then(|id| Some(id * best_diff))
}

fn parse_input(input: &File) -> (usize, Vec<String>) {
    let mut lines = BufReader::new(input).lines();
    (lines.next().unwrap().unwrap().parse().unwrap(), lines.next().unwrap().unwrap().split(",").map(str::to_owned).collect())
}
