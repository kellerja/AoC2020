use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn connect_adapters_greedily(sorted_jolts: &[usize]) -> Option<[usize; 3]> {
    if sorted_jolts.is_empty() {
        return None;
    }
    let mut differences = [0, 0, 0];

    let mut add_difference = |diff| {
        let index = match diff { 1 => Some(0), 2 => Some(1), 3 => Some(2), _ => None };
        index.and_then(|index| {
            differences[index] += 1;
            Some(differences[index])
        })
    };

    // difference between outlet (0 jolts) and first adapter
    add_difference(*sorted_jolts.first().unwrap())?;

    for jolts in sorted_jolts.windows(2) {
        let prev = jolts[0];
        let current = jolts[1];
        add_difference(current - prev)?;
    }
    // difference between device (highest adapter + 3) and highest adapter
    add_difference(3)?;

    Some(differences)
}

pub fn solve(input: &File) -> Option<usize> {
    let mut numbers = parse_input(input);
    numbers.sort_by(|one, other| one.cmp(&other));
    connect_adapters_greedily(&numbers).and_then(|[diff_1, _diff_2, diff_3]| Some(diff_1 * diff_3))
}

fn parse_input(input: &File) -> Vec<usize> {
    BufReader::new(input).lines().map(|line| *&line.unwrap().parse().unwrap()).collect()
}
