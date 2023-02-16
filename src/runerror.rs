use std::{
    io,
    num::ParseIntError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("Unable to parse {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Unable to parse {0}")]
    ParseString(String),

    #[error("{0} is not yet implemented")]
    NotImplemented(String),

    #[error("Invalid part number specified")]
    BadPartNum,

    #[error("Puzzle solver failed to run")]
    PartFailed,

    #[error("Input value out of bounds")]
    InputBounds,

    #[error("Unable to read file: {0}")]
    IO(#[from] io::Error)
}
