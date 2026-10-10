use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessError {
    #[error("{arg:?} was out of bounds. Should be between {low_bound:?} and {high_bound:?}." )]
    PositionOutOfBounds {
        arg: String,
        low_bound: String,
        high_bound: String
    },
    #[error("{item:?} was not found at {index:?}")]
    BadIndexRemove {
        item: String,
        index: String
    },
    #[error("Item `{added:?}` could not be added, as item `{present:?}` was already present.")]
    BadAdd {
        added: String,
        present: String
    }
}