use std::{any::type_name, num::TryFromIntError};

use derive_more::Add;
use serde::{Deserialize, Serialize};

use crate::errors::illegal_from_error::IllegalFromError;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

impl From<(u32, u32)> for Position {
    fn from(coords: (u32, u32)) -> Self {
        Self {x: coords.0, y: coords.1}
    }
}

impl TryFrom<PositionDelta> for Position {
    type Error = TryFromIntError;

    fn try_from(value: PositionDelta) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;
        Ok(Self{x, y})
    }
}

#[derive(Add, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub struct PositionDelta {
    pub x: i32,
    pub y: i32
}


impl From<(i32, i32)> for PositionDelta {
    fn from(coords: (i32, i32)) -> Self {
        Self {x: coords.0, y: coords.1}
    }
}

impl TryFrom<Position> for PositionDelta {
    type Error = TryFromIntError;

    fn try_from(value: Position) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;
        Ok(Self{x, y})
    }
}

impl std::ops::Add<PositionDelta> for Position {
    type Output = Position;
    fn add(self, rhs: PositionDelta) -> Self::Output {
        Position {
            x: self.x.wrapping_add_signed(rhs.x), 
            y: self.y.wrapping_add_signed(rhs.y)
        }
    }
}

impl std::ops::Add<Position> for PositionDelta {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        rhs + self   
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = PositionDelta;
    fn sub(self, rhs: Position) -> Self::Output {
        PositionDelta {
            x: (self.x as i32).wrapping_sub(rhs.x as i32),
            y: (self.y as i32).wrapping_sub(rhs.y as i32)
        }  
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

pub const ALL_DIRS: [Dir; 8] = [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW];
pub const CARDINAL_DIRS: [Dir; 4] = [Dir::N, Dir::E, Dir::S,Dir::W];