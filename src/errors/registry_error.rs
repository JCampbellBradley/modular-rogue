use std::any::TypeId;

use thiserror::Error;

use crate::mod_registry::registry::ComponentEntry;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("The tag `{tag}` was used to register two distinct components: {entry_first:?} and {entry_second:?}")]
    DuplicateComponentTag {
        tag: String,
        entry_first: ComponentEntry,
        entry_second: ComponentEntry
    },
    #[error("The component with type {id:?} was registered twice, with tags: {tag_first:?} and {tag_second:?}")]
    DuplicateComponentType {
        id: TypeId,
        tag_first: String,
        tag_second: String
    },
    #[error("Tried to serialize using unknown type `{id:?}`")]
    SerializedUnknownType {
        id: TypeId
    },
    #[error("Tried to deserialize using unknown tag `{tag:?}`")]
    DeserializedUnknownTag {
        tag: String
    },
    #[error("RonError: {0}")]
    RonError(#[from] ron::Error),
    #[error("SpannedError: {0}")]
    SpannedError(#[from] ron::error::SpannedError)
}