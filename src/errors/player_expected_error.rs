use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug)]
pub struct PlayerExpectedError {
}

impl Display for PlayerExpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The player was expected to be existent, but was not.")
    }
}

impl Error for PlayerExpectedError {}