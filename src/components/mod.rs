mod actor;
mod health;
mod keep_between_floors;

pub use actor::*;
pub use health::*;
pub use keep_between_floors::*;

use bevy::math::IVec2;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_offset(&self) -> IVec2 {
        match self {
            Self::Up => IVec2::Y,
            Self::Down => -IVec2::Y,
            Self::Left => -IVec2::X,
            Self::Right => IVec2::X,
        }
    }
}
