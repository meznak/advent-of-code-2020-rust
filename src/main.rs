mod puzzles;

use std::{
    fs,
    path,
    process::exit,
};
use clap::Parser;

#[derive(Debug)]
pub enum RunError {
    Parse,
    NotImplemented,
    BadPartNum,
    PartFailed,
}

#[derive(Parser, Debug)]
struct Args {
    // Day's puzzle
    #[arg(short, long)]
    day: u8,

    // Puzzle part
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

struct ParsedArgs {
    day: String,
    part: u8,
}

fn main() {
    let parsed_args = parse_args().unwrap();
    let data = get_data(&parsed_args.day).unwrap();

    let result = match &*parsed_args.day.clone() {
        "day01" => {
            puzzles::day01::main(parsed_args.part, data)
        },
        _ => {Err(RunError::NotImplemented)}
    };

    match result {
        Ok(_) => (),
        Err(RunError::NotImplemented) => {
            println!("That day is not yet implemented");
            exit(1);
        }
        Err(RunError::Parse) => {
            println!("Couldn't parse data");
            exit(2);
        }
        Err(RunError::BadPartNum) => {
            println!("Invalid part number specified");
            exit(3);
        }
        Err(RunError::PartFailed) => {
            println!("Puzzle failed to run")
        }
    }
}

fn parse_args<'a>() -> Result<ParsedArgs, ()> {
    let args = Args::parse();

    if args.part < 1 || args.part > 2 {
        println!("Part number must be 1 or 2");
        return Err(());
    }

    let day = format!("day{:02}", args.day);

    Ok(ParsedArgs {day: day, part: args.part})
}

fn get_data(day: &str) -> Result<String, std::io::Error> {
    let data_path = path::Path::new("data").join(day);
    fs::read_to_string(data_path)
}