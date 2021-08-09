use crate::bundles::SpriteBundleExt;
use bevy::prelude::{Bundle, SpriteBundle};

#[derive(Bundle)]
pub struct Floor {
    #[bundle]
    sprite: SpriteBundle,
}

impl Floor {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            sprite: SpriteBundle::new_background("floor.png", x, y),
        }
    }
}
