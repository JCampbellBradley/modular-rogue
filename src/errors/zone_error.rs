
use thiserror::Error;

use crate::worlds::position::Position;

#[derive(Error, Debug)]
pub enum ZoneError {
    #[error("`{arg:?}` was out of bounds. Should be between `{low_bound:?}` and `{high_bound:?}`." )]
    PositionOutOfBounds {
        arg: Position,
        low_bound: Position,
        high_bound: Position
    },
    #[error("`{item}` was not found at {index:?}")]
    BadIndexRemove {
        item: String,
        index: Position
    }
}