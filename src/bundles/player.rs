use crate::actions::{Action, DamageAction, MoveAction};
use crate::bundles::SpriteBundleExt;
use crate::components::{
    Actor, Brain, Direction, GridPosition, Health, KeepBetweenFloors, TurnGroup,
};
use crate::world::ImmutableWorld;
use bevy::input::Input;
use bevy::math::IVec2;
use bevy::prelude::{Bundle, Entity, KeyCode, SpriteBundle};
use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Bundle)]
pub struct Player {
    position: GridPosition,
    health: Health,
    actor: Actor,
    #[bundle]
    sprite: SpriteBundle,
    kbf: KeepBetweenFloors,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: GridPosition::new(x, y),
            health: Health::new(30),
            actor: Actor::new_unlimited_decision_attempts(
                PlayerBrain::WaitingForAnyInput,
                TurnGroup::Player,
            ),
            sprite: SpriteBundle::new("soul_spectre.png", x, y),
            kbf: KeepBetweenFloors,
        }
    }
}

enum PlayerBrain {
    WaitingForAnyInput,
    BufferingMovement { last_move_time: Instant },
    MovingUnbuffered,
    WaitingForAttackDirection,
}

impl Brain for PlayerBrain {
    fn decide_action(
        &mut self,
        this_entity: Entity,
        world: &mut ImmutableWorld,
    ) -> Option<Box<dyn Action>> {
        let move_up_action = MoveAction {
            entity: this_entity,
            direction: Direction::Up,
        }
        .to_brain_decision_if_can_attempt(world);
        let move_down_action = MoveAction {
            entity: this_entity,
            direction: Direction::Down,
        }
        .to_brain_decision_if_can_attempt(world);
        let move_left_action = MoveAction {
            entity: this_entity,
            direction: Direction::Left,
        }
        .to_brain_decision_if_can_attempt(world);
        let move_right_action = MoveAction {
            entity: this_entity,
            direction: Direction::Right,
        }
        .to_brain_decision_if_can_attempt(world);

        let this_position = world.get::<GridPosition>(this_entity).unwrap().clone();
        let attack_up_action = world
            .query::<(Entity, &Health, &GridPosition)>()
            .iter(world)
            .find(|(_, _, position)| this_position.add(IVec2::new(0, 1)).eq(position))
            .map(|(target, _, _)| DamageAction { damage: 10, target })
            .map(|action| action.to_brain_decision_if_can_attempt(world))
            .flatten();
        let attack_down_action = world
            .query::<(Entity, &Health, &GridPosition)>()
            .iter(world)
            .find(|(_, _, position)| this_position.add(IVec2::new(0, -1)).eq(position))
            .map(|(target, _, _)| DamageAction { damage: 10, target })
            .map(|action| action.to_brain_decision_if_can_attempt(world))
            .flatten();
        let attack_left_action = world
            .query::<(Entity, &Health, &GridPosition)>()
            .iter(world)
            .find(|(_, _, position)| this_position.add(IVec2::new(-1, 0)).eq(position))
            .map(|(target, _, _)| DamageAction { damage: 10, target })
            .map(|action| action.to_brain_decision_if_can_attempt(world))
            .flatten();
        let attack_right_action = world
            .query::<(Entity, &Health, &GridPosition)>()
            .iter(world)
            .find(|(_, _, position)| this_position.add(IVec2::new(1, 0)).eq(position))
            .map(|(target, _, _)| DamageAction { damage: 10, target })
            .map(|action| action.to_brain_decision_if_can_attempt(world))
            .flatten();

        let keyboard = world.get_resource::<Input<KeyCode>>().unwrap();

        let (new_state, action) = match self {
            Self::WaitingForAnyInput => {
                let buffering_movement = Self::BufferingMovement {
                    last_move_time: Instant::now(),
                };
                if keyboard.just_pressed(KeyCode::Key1) {
                    (Self::WaitingForAttackDirection, None)
                } else if keyboard.pressed(KeyCode::W) && move_up_action.is_some() {
                    (buffering_movement, move_up_action)
                } else if keyboard.pressed(KeyCode::A) && move_left_action.is_some() {
                    (buffering_movement, move_left_action)
                } else if keyboard.pressed(KeyCode::S) && move_down_action.is_some() {
                    (buffering_movement, move_down_action)
                } else if keyboard.pressed(KeyCode::D) && move_right_action.is_some() {
                    (buffering_movement, move_right_action)
                } else {
                    (Self::WaitingForAnyInput, None)
                }
            }

            // After the player has moved 1 tile, wait for 300ms before allowing additional movement
            Self::BufferingMovement { last_move_time } => {
                let buffering_movement = Self::BufferingMovement {
                    last_move_time: *last_move_time,
                };
                if keyboard.just_pressed(KeyCode::Key1) {
                    (Self::WaitingForAttackDirection, None)
                } else if keyboard.pressed(KeyCode::W) && move_up_action.is_some() {
                    if last_move_time.elapsed() >= Duration::from_millis(300) {
                        (Self::MovingUnbuffered, move_up_action)
                    } else {
                        (buffering_movement, None)
                    }
                } else if keyboard.pressed(KeyCode::A) && move_left_action.is_some() {
                    if last_move_time.elapsed() >= Duration::from_millis(300) {
                        (Self::MovingUnbuffered, move_left_action)
                    } else {
                        (buffering_movement, None)
                    }
                } else if keyboard.pressed(KeyCode::S) && move_down_action.is_some() {
                    if last_move_time.elapsed() >= Duration::from_millis(300) {
                        (Self::MovingUnbuffered, move_down_action)
                    } else {
                        (buffering_movement, None)
                    }
                } else if keyboard.pressed(KeyCode::D) && move_right_action.is_some() {
                    if last_move_time.elapsed() >= Duration::from_millis(300) {
                        (Self::MovingUnbuffered, move_right_action)
                    } else {
                        (buffering_movement, None)
                    }
                } else {
                    (Self::WaitingForAnyInput, None)
                }
            }

            Self::MovingUnbuffered => {
                if keyboard.just_pressed(KeyCode::Key1) {
                    (Self::WaitingForAttackDirection, None)
                } else if keyboard.pressed(KeyCode::W) && move_up_action.is_some() {
                    (Self::MovingUnbuffered, move_up_action)
                } else if keyboard.pressed(KeyCode::A) && move_left_action.is_some() {
                    (Self::MovingUnbuffered, move_left_action)
                } else if keyboard.pressed(KeyCode::S) && move_down_action.is_some() {
                    (Self::MovingUnbuffered, move_down_action)
                } else if keyboard.pressed(KeyCode::D) && move_right_action.is_some() {
                    (Self::MovingUnbuffered, move_right_action)
                } else {
                    (Self::WaitingForAnyInput, None)
                }
            }

            Self::WaitingForAttackDirection => {
                if keyboard.just_pressed(KeyCode::Key1) {
                    (Self::WaitingForAnyInput, None)
                } else if keyboard.pressed(KeyCode::W) && attack_up_action.is_some() {
                    (Self::WaitingForAnyInput, attack_up_action)
                } else if keyboard.pressed(KeyCode::A) && attack_left_action.is_some() {
                    (Self::WaitingForAnyInput, attack_left_action)
                } else if keyboard.pressed(KeyCode::S) && attack_down_action.is_some() {
                    (Self::WaitingForAnyInput, attack_down_action)
                } else if keyboard.pressed(KeyCode::D) && attack_right_action.is_some() {
                    (Self::WaitingForAnyInput, attack_right_action)
                } else {
                    (Self::WaitingForAttackDirection, None)
                }
            }
        };

        *self = new_state;
        action
    }
}
