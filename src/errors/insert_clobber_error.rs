use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug)]
pub struct InsertClobberError<T: Debug> {
    pub argument: T
}

impl<T: Debug> Display for InsertClobberError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let argument = &self.argument;
        write!(f, "Argument `{argument:?}' cannot be inserted, as it is already present.")
    }
}

impl<T: Debug> Error for InsertClobberError<T> {}