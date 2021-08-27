use crate::bundles::SpriteBundleExt;
use bevy::math::{ivec2, IVec2};
use bevy::prelude::{Bundle, SpriteBundle};

#[derive(Bundle)]
pub struct Wall {
    position: IVec2,
    #[bundle]
    sprite: SpriteBundle,
}

impl Wall {
    pub fn new(x: i32, y: i32, variant: bool) -> Self {
        let sprite = if variant { "wall1.png" } else { "wall2.png" };
        Self {
            position: ivec2(x, y),
            sprite: SpriteBundle::new(sprite, x, y),
        }
    }
}
