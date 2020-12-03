use std::fs::File;
use std::fmt;

mod day1;
mod day2;
mod day3;

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
    print_result("Day 3", &day3::solve(&input))
}

fn main() {
    solve_day3();
}
