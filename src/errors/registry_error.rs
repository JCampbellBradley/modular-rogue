use thiserror::Error;

use crate::mod_registry::registry::ComponentEntry;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("The tag `{tag}` was used to register two distinct components: {entry_first:?} and {entry_second:?}")]
    DuplicateComponentTag {
        tag: String,
        entry_first: Box<ComponentEntry>,
        entry_second: Box<ComponentEntry>
    },
    #[error("The component with type {type_name:?} was registered twice, with tags: {tag_first:?} and {tag_second:?}")]
    DuplicateComponentType {
        type_name: String,
        tag_first: String,
        tag_second: String
    },
    #[error("Tried to serialize using unknown type {type_name:?}")]
    SerializedUnknownType {
        type_name: String
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