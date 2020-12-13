use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

pub struct GreedyConnectionDiffMultiplication;
pub struct ConnectionCombinationCount;

pub trait Count {
    fn count(&self, sorted_jolts: &[usize]) -> Option<usize>;
}

impl GreedyConnectionDiffMultiplication {
    fn connect_adapters_greedily(&self, sorted_jolts: &[usize]) -> Option<[usize; 3]> {
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
}

impl Count for GreedyConnectionDiffMultiplication {
    fn count(&self, sorted_jolts: &[usize]) -> Option<usize> {
        self.connect_adapters_greedily(sorted_jolts).and_then(|[diff_1, _diff_2, diff_3]| Some(diff_1 * diff_3))
    }
}

impl ConnectionCombinationCount {
    fn combination_count_at(&self, sorted_jolts: &[usize], idx: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(value) = cache.get(&idx) {
            return *value;
        } else if idx + 1 == sorted_jolts.len() {
            return 1;
        }
        let current = sorted_jolts.get(idx).unwrap();
        let mut combination_count = 0;
        for (i, value) in sorted_jolts[idx + 1..(idx + 4).min(sorted_jolts.len())].iter().enumerate() {
            if current + 3 >= *value {
                combination_count += self.combination_count_at(sorted_jolts, idx + i + 1, cache);
            } else {
                break;
            }
        }
        cache.insert(idx, combination_count);
        combination_count
    }
}

impl Count for ConnectionCombinationCount {
    fn count(&self, sorted_jolts: &[usize]) -> Option<usize> {
        let mut numbers = sorted_jolts.to_owned();
        numbers.insert(0, 0);
        Some(self.combination_count_at(&numbers, 0, &mut HashMap::with_capacity(numbers.len())))
    }
}

pub fn solve(input: &File, counter: &impl Count) -> Option<usize> {
    let mut numbers = parse_input(input);
    numbers.sort_by(|one, other| one.cmp(&other));
    counter.count(&numbers)
}

fn parse_input(input: &File) -> Vec<usize> {
    BufReader::new(input).lines().map(|line| *&line.unwrap().parse().unwrap()).collect()
}
