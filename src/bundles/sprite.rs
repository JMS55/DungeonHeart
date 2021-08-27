use bevy::math::{vec2, vec3};
use bevy::prelude::{Handle, SpriteBundle, Transform};
use bevy::sprite::{ColorMaterial, Sprite};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};

pub trait SpriteBundleExt {
    fn new(sprite: &str, x: i32, y: i32) -> Self;
    fn new_background(sprite: &str, x: i32, y: i32) -> Self;
}

impl SpriteBundleExt for SpriteBundle {
    fn new(sprite: &str, x: i32, y: i32) -> Self {
        Self {
            sprite: Sprite::new(vec2(32.0, 32.0)),
            material: MaterialMap::get(sprite),
            transform: Transform {
                translation: vec3((x * 32) as f32, (y * 32) as f32, 1.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn new_background(sprite: &str, x: i32, y: i32) -> Self {
        Self {
            sprite: Sprite::new(vec2(32.0, 32.0)),
            material: MaterialMap::get(sprite),
            transform: Transform {
                translation: vec3((x * 32) as f32, (y * 32) as f32, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub static MATERIAL_MAP: MaterialMap = MaterialMap(OnceCell::new());

pub struct MaterialMap(pub OnceCell<HashMap<OsString, Handle<ColorMaterial>>>);

impl MaterialMap {
    fn get(key: &str) -> Handle<ColorMaterial> {
        MATERIAL_MAP
            .0
            .get()
            .unwrap()
            .get(OsStr::new(key))
            .unwrap()
            .clone_weak()
    }
}
