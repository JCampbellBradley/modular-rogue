
use ron::de::SpannedError;
use thiserror::Error;

use crate::errors::{entity_error::EntityError, registry_error::RegistryError};

#[derive(Error, Debug)]
pub enum BakeError {
    #[error("{0:?}")]
    RonError(#[from] ron::Error),
    #[error("{0:?}")]
    SpannedError(#[from] SpannedError),
    #[error("{0:?}")]
    RegistryError(#[from] RegistryError),
    #[error("{0:?}")]
    EntityError(#[from] EntityError)
}