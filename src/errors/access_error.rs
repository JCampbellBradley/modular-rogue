use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessError<A, I> {
    #[error("{arg:?} was out of bounds. Should be between {low_bound:?} and {high_bound:?}." )]
    PositionOutOfBounds {
        arg: A,
        low_bound: A,
        high_bound: A
    },
    #[error("{item:?} was not found at {index:?}")]
    BadIndexRemove {
        item: I,
        index: A
    },
    #[error("Item {item:?} could not be added, as it was already present.")]
    BadAdd {
        item: I
    }
}