use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug)]
pub struct IllegalArgumentError<T: Debug> {
    pub argument_name: &'static str,
    pub argument: T,
    pub reason: &'static str
}

impl<T: Debug> Display for IllegalArgumentError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let argument_name = self.argument_name;
        let argument = &self.argument;
        let reason= self.reason;
        write!(f, "Argument `{argument_name} = {argument:?}' is not valid for this function.\n\
                    Reason: {reason}.")
    }
}

impl<T: Debug> Error for IllegalArgumentError<T> {}