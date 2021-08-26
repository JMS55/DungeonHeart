use bevy::math::IVec2;
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Eq)]
pub struct GridPosition(IVec2);

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }
}

impl Deref for GridPosition {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GridPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
