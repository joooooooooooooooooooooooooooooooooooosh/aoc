use std::env;
use std::io;
use std::fs;
use std::error::Error;
use std::convert::TryFrom;

use aoc::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let problem_num: u8;
    if args.len() > 1 {
        problem_num = match args[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", args[1]);
                return Ok(())
            }
        };
    } else {
        problem_num = find_latest_problem()?;
        println!("No problem specified, defaulting to {}", problem_num);
    }

    run_problem(problem_num);

    Ok(())
}

// Assumptions: one input file for each problem
fn find_latest_problem() -> Result<u8, Box<dyn Error>> {
    let entries = fs::read_dir("input")?
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(u8::try_from(entries.len())?)
}
