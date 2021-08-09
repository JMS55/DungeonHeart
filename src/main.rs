use crate::actions::RegenerateDungeonAction;
use crate::components::KeepBetweenFloors;
use crate::world::WorldExt;
use actions::{perform_next_action, ActionStack};
use bevy::prelude::{
    App, AssetServer, Assets, BuildWorldChildren, ClearColor, Color, IntoExclusiveSystem,
    IntoSystem, OrthographicCameraBundle, Transform, World,
};
use bevy::sprite::ColorMaterial;
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use bundles::{Player, SkeletonScout, MATERIAL_MAP};
use components::{decide_next_action, determine_turn_group, TurnGroup};
use std::collections::HashMap;

mod actions;
mod bundles;
mod components;
mod world;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 480.0,
            height: 480.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .insert_resource(ActionStack::new())
        .insert_resource(TurnGroup::Neutral)
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_game.exclusive_system())
        .add_system(determine_turn_group.system())
        .add_system(decide_next_action.exclusive_system())
        .add_system(perform_next_action.exclusive_system())
        .run();
}

fn init_game(world: &mut World) {
    let assets = world.get_resource::<AssetServer>().unwrap();
    #[cfg(debug_assertions)]
    assets.watch_for_changes().unwrap();
    let mut materials =
        unsafe { world.get_resource_unchecked_mut::<Assets<ColorMaterial>>() }.unwrap();
    let mut material_map = HashMap::new();
    // TODO: Autoload entire folder
    for material in [
        "floor.png",
        "skeleton_scout.png",
        "soul_spectre.png",
        "wall_mossy.png",
        "wall.png",
    ] {
        material_map.insert(material, materials.add(assets.load(material).into()));
    }
    MATERIAL_MAP.map.set(material_map).unwrap();

    world
        .spawn()
        .insert_bundle(SkeletonScout::new(0, 0))
        .insert(KeepBetweenFloors);
    world
        .spawn()
        .insert_bundle(SkeletonScout::new(1, 0))
        .insert(KeepBetweenFloors);
    world
        .spawn()
        .insert_bundle(SkeletonScout::new(0, 1))
        .insert(KeepBetweenFloors);
    world
        .spawn()
        .insert_bundle(SkeletonScout::new(1, 1))
        .insert(KeepBetweenFloors);
    world
        .spawn()
        .insert_bundle(Player::new(2, 2))
        .with_children(|player| {
            let mut camera = player.spawn_bundle(OrthographicCameraBundle::new_2d());
            camera.insert(KeepBetweenFloors);
            camera.get_mut::<Transform>().unwrap().translation.z -= 1.0; // TODO: See if this is needed to make the floor render in future bevy releases
        });

    world.add_action(RegenerateDungeonAction::new());
}
