use aoc::*;
use std::env;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or("0".to_string())
        .parse::<i32>()
        .expect("Day must be a number");


    match day {
        0 => run_all_days(),
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        _ => panic!("Specified day is not available."),
    };
}

fn run_all_days() {
    day01::run();
    day02::run();
    day03::run();
}
