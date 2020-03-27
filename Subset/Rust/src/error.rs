use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CustomError {
    Parse,
    InvalidNumber,
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Parse => {
                write!(f, "An parse error occured")
            },
            CustomError::InvalidNumber => {
                write!(f, "The number must be between 1 and 20!")
            },
        }
    }
}