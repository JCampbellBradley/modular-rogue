use std::any::type_name;

use serde::{Deserialize, Serialize};

use crate::{errors::illegal_from_error::IllegalFromError, worlds::position::PositionDelta};


#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    ORIGIN
}

impl Dir {
    pub fn to_delta(&self) -> PositionDelta {
        match self {
            Dir::N => (0, 1),
            Dir::NE => (1, 1),
            Dir::E => (1, 0),
            Dir::SE => (1, -1),
            Dir::S => (0, -1),
            Dir::SW => (-1, -1),
            Dir::W => (-1, 0),
            Dir::NW => (-1, 1),
            Dir::ORIGIN => (0, 0)
        }.into()
    }
}

impl From<DiagonalDir> for Dir {
    fn from(value: DiagonalDir) -> Self {
        match value {
            DiagonalDir::NE => Dir::NE,
            DiagonalDir::NW => Dir::NW,
            DiagonalDir::SW => Dir::SW,
            DiagonalDir::SE => Dir::SE,
        }
    }
}

impl From<CardinalDir> for Dir {
    fn from(value: CardinalDir) -> Self {
        match value {
            CardinalDir::N => Dir::N,
            CardinalDir::E => Dir::E,
            CardinalDir::S => Dir::S,
            CardinalDir::W => Dir::W,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DiagonalDir {
    NE,
    NW,
    SW,
    SE
}

impl DiagonalDir {
    pub fn orthant_from_delta(delta: &PositionDelta) -> Self {
        match (delta.x >= 0, delta.y >= 0) {
            (true, true) => DiagonalDir::NE,
            (true, false) => DiagonalDir::SE,
            (false, false) => DiagonalDir::SW,
            (false, true) => DiagonalDir::NW,
        }
    }
}

impl TryFrom<Dir> for DiagonalDir {
    type Error = IllegalFromError<Dir>;
    fn try_from(value: Dir) -> Result<Self, Self::Error> {
        match value {
            Dir::NE => Ok(DiagonalDir::NE),
            Dir::SE => Ok(DiagonalDir::SE),
            Dir::SW => Ok(DiagonalDir::SW),
            Dir::NW => Ok(DiagonalDir::NW),
            _ => Err(IllegalFromError::<Dir> {
                argument: value,
                into_type_name: type_name::<Self>()
            })
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CardinalDir {
    N,
    E,
    S,
    W
}

impl TryFrom<Dir> for CardinalDir {
    type Error = IllegalFromError<Dir>;
    fn try_from(value: Dir) -> Result<Self, Self::Error> {
        match value {
            Dir::N => Ok(CardinalDir::N),
            Dir::E => Ok(CardinalDir::E),
            Dir::S => Ok(CardinalDir::S),
            Dir::W => Ok(CardinalDir::W),
            _ => Err(IllegalFromError::<Dir> {
                argument: value,
                into_type_name: type_name::<Self>()
            })
        }
    }
}

pub const ALL_DIRS: [Dir; 9] = [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW, Dir::ORIGIN];
pub const ADJACENT_DIRS: [Dir; 8] = [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW];
pub const CARDINAL_DIRS: [CardinalDir; 4] = [CardinalDir::N, CardinalDir::E, CardinalDir::S,CardinalDir::W];
pub const DIAGONAL_DIRS: [DiagonalDir; 4] = [DiagonalDir::NE, DiagonalDir::SE, DiagonalDir::SW,DiagonalDir::NW];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn dir_to_delta() {
        let dirs: HashSet<PositionDelta> = ALL_DIRS.into_iter()
            .map(|dir| dir.to_delta())
            .collect();

        assert_eq!(dirs.iter().count(), 9);
        assert_eq!(dirs.into_iter().reduce(|a, b| a+b).unwrap() , (0, 0).into());
    }

    #[test]
    fn dir_to_from_diagonal() {
        for diag in DIAGONAL_DIRS {
            let dir: Dir = diag.into();
            assert_eq!(diag, dir.try_into().unwrap())
        }
    }

    #[test]
    fn diagonal_to_dir_err() {
        for card in CARDINAL_DIRS {
            let dir: Dir = card.into();
            let diag: Result<DiagonalDir, _> = dir.try_into();
            assert!(diag.is_err())
        }
    }

    #[test]
    fn cardinal_to_dir_err() {
        for card in DIAGONAL_DIRS {
            let dir: Dir = card.into();
            let diag: Result<CardinalDir, _> = dir.try_into();
            assert!(diag.is_err())
        }
    }

    #[test]
    fn dir_to_from_cardinal() {
        for card in CARDINAL_DIRS {
            let dir: Dir = card.into();
            assert_eq!(card, dir.try_into().unwrap())
        }
    }
}