use std::fs::File;
use std::fmt;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn open_file(filename: &str) -> File {
    File::open(&filename).expect(&format!("Could not open file {}", filename))
}

fn get_filename(day: &str) -> String {
    format!("input/{}/input.txt", day)
}

fn print_result(tag: &str, result: &Option<impl fmt::Display>) {
    match result {
        Some(result) => println!("{} solution: {}", tag, result),
        None => println!("{} input has no solution", tag)
    }
}

pub fn solve_day1() {
    let input = open_file(&get_filename("day1"));
    print_result("Day 1", &day1::solve(&input, 2))
}

pub fn solve_day1_part_2() {
    let input = open_file(&get_filename("day1"));
    print_result("Day 1 part 2", &day1::solve(&input, 3))
}

pub fn solve_day2() {
    let input = open_file(&get_filename("day2"));
    print_result("Day 2", &day2::solve(&input, day2::CharCountPolicy))
}

pub fn solve_day2_part_2() {
    let input = open_file(&get_filename("day2"));
    print_result("Day 2 part 2", &day2::solve(&input, day2::CharPositionPolicy))
}

pub fn solve_day3() {
    let input = open_file(&get_filename("day3"));
    print_result("Day 3", &day3::solve(&input, &day3::PART_1_SLOPES.to_vec()))
}

pub fn solve_day3_part_2() {
    let input = open_file(&get_filename("day3"));
    print_result("Day 3 part 2", &day3::solve(&input, &day3::PART_2_SLOPES.to_vec()))
}

pub fn solve_day4() {
    let input = open_file(&get_filename("day4"));
    print_result("Day 4", &day4::solve(&input, day4::FieldPresenceValidator))
}

pub fn solve_day4_part_2() {
    let input = open_file(&get_filename("day4"));
    print_result("Day 4 part 2", &day4::solve(&input, day4::FieldValueValidator))
}

pub fn solve_day5() {
    let input = open_file(&get_filename("day5"));
    print_result("Day 5", &day5::solve(&input, day5::SearchCriteria::HIGHEST))
}

pub fn solve_day5_part_2() {
    let input = open_file(&get_filename("day5"));
    print_result("Day 5 part 2", &day5::solve(&input, day5::SearchCriteria::EMPTY))
}

pub fn solve_day6() {
    let input = open_file(&get_filename("day6"));
    print_result("Day 6", &day6::solve(&input, &day6::AnyonesAnswer))
}

pub fn solve_day6_part_2() {
    let input = open_file(&get_filename("day6"));
    print_result("Day 6 part 2", &day6::solve(&input, &day6::EveryonesAnswer))
}

pub fn solve_day7() {
    let input = open_file(&get_filename("day7"));
    print_result("Day 7", &day7::solve(&input, &day7::UniqueRootsCounter))
}

pub fn solve_day7_part_2() {
    let input = open_file(&get_filename("day7"));
    print_result("Day 7 part 2", &day7::solve(&input, &day7::BagCapacityCounter))
}

pub fn solve_day8() {
    let input = open_file(&get_filename("day8"));
    print_result("Day 8", &day8::solve(&input, false))
}

pub fn solve_day8_part_2() {
    let input = open_file(&get_filename("day8"));
    print_result("Day 8 part 2", &day8::solve(&input, true))
}

pub fn solve_day9() {
    const PREAMBLE_SIZE: usize = 25;
    let input = open_file(&get_filename("day9"));
    print_result("Day 9", &day9::solve(&input, PREAMBLE_SIZE).0)
}

pub fn solve_day9_part_2() {
    const PREAMBLE_SIZE: usize = 25;
    let input = open_file(&get_filename("day9"));
    print_result("Day 9 part 2", &day9::solve(&input, PREAMBLE_SIZE).1)
}

fn main() {
}
