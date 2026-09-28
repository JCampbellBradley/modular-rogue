use std::any::TypeId;

use thiserror::Error;

use crate::{components::component::Component, errors::{access_error::AccessError, registry_error::RegistryError}};

#[derive(Error, Debug)]
pub enum BakeError {
    #[error("{0:?}")]
    RonError(#[from] ron::Error),
    #[error("{0:?}")]
    RegistryError(#[from] RegistryError),
    #[error("{0:?}")]
    ComponentAccessError(#[from] AccessError<TypeId, Box<dyn Component>>),
}