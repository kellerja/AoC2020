extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
extern crate num_integer;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use num_integer::Integer;
use num_bigint::{ BigInt, ModInverse };
use num_traits::identities::{ Zero, One };

pub fn solve(input: &File) -> (Option<String>, Option<String>) {
    let (earliest_time, schedule) = parse_input(input);
    let initial_timestamps: Vec<(BigInt, BigInt)> = schedule.iter().enumerate()
        .filter(|(_, time)| time != &"x")
        .map(|(pos, time)| (BigInt::from(pos), BigInt::from(time.parse::<usize>().unwrap()))).collect();
    let coprime_product: BigInt = initial_timestamps.iter().fold(One::one(), |acc, (_, time)| acc * time);
    let mut solution: BigInt = initial_timestamps.iter().map(|(pos, time)| {
        let g: BigInt = &coprime_product / time;
        g.clone() * g.mod_inverse(time).unwrap() * (-pos).mod_floor(time)
    }).sum();
    while &solution - &coprime_product > Zero::zero() {
        solution -= &coprime_product;
    }

    let mut best_diff = BigInt::from(usize::MAX);
    let mut id = None;
    for (_, time) in initial_timestamps {
        let diff = ((&earliest_time / &time + 1) * &time) - &earliest_time;
        if diff < best_diff {
            best_diff = diff;
            id = Some(time);
        }
    }
    (id.and_then(|id| Some((id * best_diff).to_str_radix(10))), Some(solution.to_str_radix(10)))
}

fn parse_input(input: &File) -> (BigInt, Vec<String>) {
    let mut lines = BufReader::new(input).lines();
    (lines.next().unwrap().unwrap().parse().unwrap(), lines.next().unwrap().unwrap().split(",").map(str::to_owned).collect())
}
