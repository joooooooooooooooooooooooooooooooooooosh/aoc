use std::env;
use std::fs;

pub mod day01;

pub fn run_problem(day: u8) {
    match day {
        1 => day01::day01(get_input(day)),
        _ => println!("This problem isn't implemented")
    }
}

fn get_input(day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("input").join(format!("{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}
