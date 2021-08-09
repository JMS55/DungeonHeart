use crate::actions::{Action, Direction, MoveAction};
use crate::bundles::SpriteBundleExt;
use crate::components::{Actor, Damageable, GridPosition, TurnGroup};
use crate::world::ImmutableWorld;
use bevy::input::Input;
use bevy::prelude::{Bundle, Entity, KeyCode, SpriteBundle};

#[derive(Bundle)]
pub struct Player {
    position: GridPosition,
    damageable: Damageable,
    actor: Actor,
    #[bundle]
    sprite: SpriteBundle,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: GridPosition::new(x, y),
            damageable: Damageable::new(),
            actor: Actor::new(player_brain, TurnGroup::Player),
            sprite: SpriteBundle::new("soul_spectre.png", x, y),
        }
    }
}

fn player_brain(player_entity: Entity, world: &mut ImmutableWorld) -> Option<Box<dyn Action>> {
    let keyboard = world.get_resource::<Input<KeyCode>>().unwrap();
    if keyboard.pressed(KeyCode::W) {
        MoveAction {
            entity: player_entity,
            direction: Direction::Up,
        }
        .to_brain_decision_if_can_perform(world)
    } else if keyboard.pressed(KeyCode::A) {
        MoveAction {
            entity: player_entity,
            direction: Direction::Left,
        }
        .to_brain_decision_if_can_perform(world)
    } else if keyboard.pressed(KeyCode::S) {
        MoveAction {
            entity: player_entity,
            direction: Direction::Down,
        }
        .to_brain_decision_if_can_perform(world)
    } else if keyboard.pressed(KeyCode::D) {
        MoveAction {
            entity: player_entity,
            direction: Direction::Right,
        }
        .to_brain_decision_if_can_perform(world)
    } else {
        None
    }
}
