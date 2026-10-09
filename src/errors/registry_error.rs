use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("The tag {tag} was used to register two distinct components: {type_first:?} and {type_second:?}")]
    DuplicateComponentTag {
        tag: String,
        type_first: String,
        type_second: String
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
    #[error("Tried to deserialize using unknown tag {tag:?}")]
    DeserializedUnknownTag {
        tag: String
    },
    #[error("RonError: {0}")]
    RonError(#[from] ron::Error),
    #[error("SpannedError: {0}")]
    SpannedError(#[from] ron::error::SpannedError)
}