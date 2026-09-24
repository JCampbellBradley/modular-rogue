use thiserror::Error;


#[derive(Error, Debug)]
pub enum WorldStateError {
    #[error("The player was expected, but was not present")]
    PlayerMissing
}