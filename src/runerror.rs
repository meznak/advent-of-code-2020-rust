use std::{
    io,
    num::ParseIntError,
    string::ParseError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("Unable to parse {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Unable to parse {0}")]
    ParseString(#[from] ParseError),

    #[error("That day is not yet implemented")]
    NotImplemented,

    #[error("Invalid part number specified")]
    BadPartNum,

    #[error("Puzzle solver failed to run")]
    PartFailed,

    #[error("Unable to read file: {0}")]
    IO(#[from] io::Error)
}
