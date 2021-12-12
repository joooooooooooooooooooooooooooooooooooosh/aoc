use std::env;
use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

// stolen from spanishpear who stole it from nickyvanurk
// because i'm bad at rust imports
macro_rules! run_day {
    ($day:path, $input:expr) => {{
        use $day::*;
        println!(
            "{}: part1 = {:?}, part2 = {:?}",
            stringify!($day),
            part1($input).unwrap(),
            part2($input).unwrap()
        );
    }};
}

pub fn run_problem(day: u8) {
    match day {
        1 => run_day!(day01, get_input(day)),
        2 => run_day!(day02, get_input(day)),
        3 => run_day!(day03, get_input(day)),
        4 => run_day!(day04, get_input(day)),
        5 => run_day!(day05, get_input(day)),
        6 => run_day!(day06, get_input(day)),
        7 => run_day!(day07, get_input(day)),
        8 => run_day!(day08, get_input(day)),
        9 => run_day!(day09, get_input(day)),
        10 => run_day!(day10, get_input(day)),
        11 => run_day!(day11, get_input(day)),
        _ => println!("This problem isn't implemented")
    };
}

fn get_input(day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("input").join(format!("{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}

