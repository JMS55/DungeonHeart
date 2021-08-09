use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Handle, SpriteBundle, Transform};
use bevy::sprite::{ColorMaterial, Sprite};
use once_cell::sync::OnceCell;
use std::collections::HashMap;

pub trait SpriteBundleExt {
    fn new(sprite: &str, x: i32, y: i32) -> Self;
}

impl SpriteBundleExt for SpriteBundle {
    fn new(sprite: &str, x: i32, y: i32) -> Self {
        Self {
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            material: MaterialMap::get(sprite),
            transform: Transform {
                translation: Vec3::new((x * 32) as f32, (y * 32) as f32, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub static MATERIAL_MAP: MaterialMap = MaterialMap {
    map: OnceCell::new(),
};

pub struct MaterialMap {
    pub map: OnceCell<HashMap<&'static str, Handle<ColorMaterial>>>,
}

impl MaterialMap {
    fn get(key: &str) -> Handle<ColorMaterial> {
        MATERIAL_MAP
            .map
            .get()
            .unwrap()
            .get(key)
            .unwrap()
            .clone_weak()
    }
}
