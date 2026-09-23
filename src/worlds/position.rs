use std::num::TryFromIntError;

use derive_more::Add;
use serde::{Deserialize, Serialize};


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_from_i32() {
        let delta: PositionDelta = (-1, 2).into();

        assert_eq!(delta.x, -1);
        assert_eq!(delta.y, 2);
    }

    #[test]
    fn position_from_u32() {
        let pos: Position = (0, 2).into();

        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 2);
    }

    #[test]
    fn position_from_delta() {
        let delta: PositionDelta = (0, 2).into();
        let pos: Position = delta.try_into().unwrap(); 

        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 2);
    }

    #[test]
    #[should_panic]
    fn position_from_delta_err() {
        let delta: PositionDelta = (-1, 2).into();
        let _: Position = delta.try_into().unwrap();
    }

    #[test]
    fn delta_from_position() {
        let pos: Position = (0, 2).into();
        let delta: PositionDelta = pos.try_into().unwrap();

        assert_eq!(delta.x, 0);
        assert_eq!(delta.y, 2);
    }

    #[test]
    #[should_panic]
    fn delta_from_position_err() {
        let pos: Position = (0, u32::MAX).into();
        let _: PositionDelta = pos.try_into().unwrap();
    }

    #[test]
    fn add_position_to_delta() {
        let pos: Position = (0, 2).into();
        let delta: PositionDelta = (3, -1).into();
        let sum1 = pos + delta;
        let sum2 = delta + pos;

        assert_eq!(sum1.x, 3);
        assert_eq!(sum1.y, 1);
        assert_eq!(sum1, sum2);
    }
}