use crate::bundles::SpriteBundleExt;
use crate::components::GridPosition;
use bevy::prelude::{Bundle, SpriteBundle};

#[derive(Bundle)]
pub struct Wall {
    position: GridPosition,
    #[bundle]
    sprite: SpriteBundle,
}

impl Wall {
    pub fn new(x: i32, y: i32, variant: bool) -> Self {
        let sprite = if variant { "wall1.png" } else { "wall2.png" };
        Self {
            position: GridPosition::new(x, y),
            sprite: SpriteBundle::new(sprite, x, y),
        }
    }
}
