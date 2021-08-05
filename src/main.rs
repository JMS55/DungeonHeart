use actions::{perform_next_action, ActionStack};
use bevy::prelude::{
    App, AssetServer, Assets, Commands, Handle, IntoExclusiveSystem, IntoSystem,
    OrthographicCameraBundle, ParallelSystemDescriptorCoercion, Res, ResMut,
};
use bevy::sprite::ColorMaterial;
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use bundles::{Player, SkeletonScout};
use components::{decide_next_action, determine_turn_group, TurnGroup};
use once_cell::sync::OnceCell;
use std::collections::HashMap;

mod actions;
mod bundles;
mod components;
mod immutable_world;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 480.0,
            height: 480.0,
            ..Default::default()
        })
        .insert_resource(ActionStack::new())
        .insert_resource(TurnGroup::Player)
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_material_map.system().label("init_material_map"))
        .add_startup_system(init_game.system().after("init_material_map"))
        .add_system(determine_turn_group.system())
        .add_system(decide_next_action.exclusive_system())
        .add_system(perform_next_action.exclusive_system())
        .run();
}

static MATERIAL_MAP: OnceCell<HashMap<&'static str, Handle<ColorMaterial>>> = OnceCell::new();

fn init_material_map(assets: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut material_map = HashMap::new();
    for material in ["soul_spectre.png", "skeleton_scout.png"] {
        material_map.insert(material, materials.add(assets.load(material).into()));
    }
    MATERIAL_MAP.set(material_map).unwrap();
}

fn init_game(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());

    cmd.spawn_bundle(SkeletonScout::new(0, 0));
    cmd.spawn_bundle(SkeletonScout::new(1, 0));
    cmd.spawn_bundle(SkeletonScout::new(0, 1));
    cmd.spawn_bundle(SkeletonScout::new(1, 1));
    cmd.spawn_bundle(Player::new(2, 2));
}
