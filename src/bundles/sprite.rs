use bevy::math::{Vec2, Vec3};
use bevy::prelude::{SpriteBundle, Transform};
use bevy::sprite::Sprite;

use crate::MATERIAL_MAP;

pub trait SpriteBundleExt {
    fn new(sprite: &str, x: i32, y: i32) -> Self;
}

impl SpriteBundleExt for SpriteBundle {
    fn new(sprite: &str, x: i32, y: i32) -> Self {
        Self {
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            material: MATERIAL_MAP.get().unwrap().get(sprite).unwrap().clone(),
            transform: Transform {
                translation: Vec3::new((x * 32) as f32, (y * 32) as f32, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
