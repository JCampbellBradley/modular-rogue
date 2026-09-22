use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug)]
pub struct IllegalFromError<F: Debug> {
    pub argument: F,
    pub into_type_name: &'static str
}

impl<F: Debug> Display for IllegalFromError<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let argument = &self.argument;
        let into_type_name = &self.into_type_name;
        write!(f, "Argument `{argument:?}' cannot go into type {into_type_name}.")
    }
}

impl<F: Debug> Error for IllegalFromError<F> {}