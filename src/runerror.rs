use std::{
    error,
    fmt,
    num::ParseIntError,
    io
};

#[derive(Debug)]
pub enum RunError {
    Parse(ParseIntError),
    NotImplemented,
    BadPartNum,
    PartFailed,
    IO(io::Error)
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RunError::NotImplemented =>
                write!(f, "That day is not yet implemented"),
            RunError::Parse(..) =>
                write!(f, "Couldn't parse data"),
            RunError::BadPartNum =>
                write!(f, "Invalid part number specified"),
            RunError::PartFailed =>
                write!(f, "Puzzle failed to run"),
            RunError::IO(..) =>
                write!(f, "Unable to read file"),
        }
    }
}

impl error::Error for RunError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RunError::NotImplemented => None,
            RunError::Parse(ref e) => Some(e),
            RunError::BadPartNum => None,
            RunError::PartFailed => None,
            RunError::IO(ref e) => Some(e)
        }
    }
}
