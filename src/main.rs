mod puzzles;
mod runerror;

use std::{fs, path};
use clap::Parser;
use runerror::RunError;

#[derive(Parser, Debug)]
struct Args {
    // Day's puzzle
    #[arg(short, long)]
    day: u8,

    // Puzzle part
    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

struct ParsedArgs {
    day: String,
    part: u8,
}

fn main() -> Result<(), RunError> {
    let parsed_args = parse_args()?;
    let data = get_data(&parsed_args.day)?;
    let parts: Vec<u8> = match parsed_args.part {
        0 => [1, 2].to_vec(),
        p => [p].to_vec()
    };

    for part in parts {

        let result = match &*parsed_args.day {
            "day01" => {puzzles::day01::main(part, &data)?},
            "day02" => {puzzles::day02::main(part, &data)?},
            "day03" => {puzzles::day03::main(part, &data)?},
            "day04" => {puzzles::day04::main(part, &data)?},
            "day05" => {puzzles::day05::main(part, &data)?},
            _ => {return Err(RunError::NotImplemented(parsed_args.day));}
        };

        println!("{} part {}:\n{}",
        parsed_args.day, part, result)
    }

    Ok(())
}

fn parse_args() -> Result<ParsedArgs, RunError> {
    let args = Args::parse();

    if args.part > 2 {
        return Err(RunError::BadPartNum);
    }

    let day = format!("day{:02}", args.day);

    Ok(ParsedArgs {day, part: args.part})
}

fn get_data(day: &str) -> Result<String, RunError> {
    let data_path = path::Path::new("data").join(day);
    match fs::read_to_string(data_path) {
        Ok(data) => Ok(data.replace("\r\n", "\n")),
        Err(e) => Err(RunError::IO(e))
    }
}
