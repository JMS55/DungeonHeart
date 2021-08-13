use crate::actions::{Action, Direction, MoveAction};
use crate::bundles::SpriteBundleExt;
use crate::components::{Actor, Brain, Damageable, GridPosition, KeepBetweenFloors, TurnGroup};
use crate::world::ImmutableWorld;
use bevy::input::Input;
use bevy::prelude::{Bundle, Entity, KeyCode, SpriteBundle};
use std::time::{Duration, Instant};

#[derive(Bundle)]
pub struct Player {
    position: GridPosition,
    damageable: Damageable,
    actor: Actor,
    #[bundle]
    sprite: SpriteBundle,
    kbf: KeepBetweenFloors,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: GridPosition::new(x, y),
            damageable: Damageable::new(),
            actor: Actor::new(PlayerBrain::CanMoveOnce, TurnGroup::Player),
            sprite: SpriteBundle::new("soul_spectre.png", x, y),
            kbf: KeepBetweenFloors,
        }
    }
}

#[derive(Clone)]
enum PlayerBrain {
    CanMoveOnce,
    MovedOnce { when: Instant },
    MovingMany,
}

impl PlayerBrain {
    fn decide_action(this_entity: Entity, world: &mut ImmutableWorld) -> Option<Box<dyn Action>> {
        let keyboard = world.get_resource::<Input<KeyCode>>().unwrap();
        if keyboard.pressed(KeyCode::W) {
            MoveAction {
                entity: this_entity,
                direction: Direction::Up,
            }
            .to_brain_decision_if_can_perform(world)
        } else if keyboard.pressed(KeyCode::A) {
            MoveAction {
                entity: this_entity,
                direction: Direction::Left,
            }
            .to_brain_decision_if_can_perform(world)
        } else if keyboard.pressed(KeyCode::S) {
            MoveAction {
                entity: this_entity,
                direction: Direction::Down,
            }
            .to_brain_decision_if_can_perform(world)
        } else if keyboard.pressed(KeyCode::D) {
            MoveAction {
                entity: this_entity,
                direction: Direction::Right,
            }
            .to_brain_decision_if_can_perform(world)
        } else {
            None
        }
    }
}

impl Brain for PlayerBrain {
    fn decide_action(
        &mut self,
        this_entity: Entity,
        world: &mut ImmutableWorld,
    ) -> Option<Box<dyn Action>> {
        // Only allow consecutive movements if trying to move for at least 300ms
        match self {
            Self::CanMoveOnce => {
                let action = Self::decide_action(this_entity, world);
                if action.is_some() {
                    *self = Self::MovedOnce {
                        when: Instant::now(),
                    };
                }
                action
            }
            Self::MovedOnce { when } => {
                let action = Self::decide_action(this_entity, world);
                if action.is_some() {
                    if when.elapsed() >= Duration::from_millis(300) {
                        *self = Self::MovingMany;
                        return action;
                    }
                } else {
                    *self = Self::CanMoveOnce;
                }
                None
            }
            Self::MovingMany => {
                let action = Self::decide_action(this_entity, world);
                if action.is_none() {
                    *self = Self::CanMoveOnce;
                }
                action
            }
        }
    }
}
