use crate::bundles::SpriteBundleExt;
use bevy::prelude::{Bundle, SpriteBundle};
use rand::{thread_rng, Rng};

#[derive(Bundle)]
pub struct Floor {
    #[bundle]
    sprite: SpriteBundle,
}

impl Floor {
    pub fn new(x: i32, y: i32) -> Self {
        let sprite = if thread_rng().gen_ratio(1, 4) {
            "floor_alt.png"
        } else {
            "floor.png"
        };
        Self {
            sprite: SpriteBundle::new_background(sprite, x, y),
        }
    }
}
