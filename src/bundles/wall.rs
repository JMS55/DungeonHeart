use crate::bundles::SpriteBundleExt;
use crate::components::GridPosition;
use bevy::prelude::{Bundle, SpriteBundle};
use rand::{thread_rng, Rng};

#[derive(Bundle)]
pub struct Wall {
    position: GridPosition,
    #[bundle]
    sprite: SpriteBundle,
}

impl Wall {
    pub fn new(x: i32, y: i32) -> Self {
        let sprite = if thread_rng().gen_ratio(1, 4) {
            "wall_mossy.png"
        } else {
            "wall.png"
        };
        Self {
            position: GridPosition::new(x, y),
            sprite: SpriteBundle::new(sprite, x, y),
        }
    }
}
