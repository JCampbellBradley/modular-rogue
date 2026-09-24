use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessError<A, I> {
    #[error("Argument {arg:?} was out of bounds {low_bound:?} to {high_bound:?}.")]
    PositionOutOfBounds {
        arg: A,
        low_bound: A,
        high_bound: A
    },
    #[error("Item {item:?} was not found at {index:?}")]
    BadIndexRemove {
        item: I,
        index: A
    },
    #[error("Item {item:?} could not be added, as it was already present.")]
    BadAdd {
        item: I
    }
}