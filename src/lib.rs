use std::env;
use std::fs;

pub mod day01;

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
        _ => println!("This problem isn't implemented")
    };
}

fn get_input(day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("input").join(format!("{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}

