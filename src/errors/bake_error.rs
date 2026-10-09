
use ron::de::SpannedError;
use thiserror::Error;

use crate::errors::{entity_error::EntityError, registry_error::RegistryError};

#[derive(Error, Debug)]
pub enum BakeError {
    #[error("Got a RON error while trying to bake blueprint.\n {0}")]
    RonError(#[from] ron::Error),
    #[error("Got a RON deserialization error while trying to bake blueprint.\n {0}")]
    SpannedError(#[from] SpannedError),
    #[error("Got a Registry error while trying to bake blueprint:\n {0}")]
    RegistryError(#[from] RegistryError),
    #[error("Got an Entity error while trying to bake blueprint:\n{0}")]
    EntityError(#[from] EntityError)
}