use crate::actions::{Action, ActionStatus};
use crate::components::GridPosition;
use crate::world::{ImmutableWorld, WorldExt};
use bevy::core::Time;
use bevy::prelude::{Entity, Transform, World};
use std::time::Duration;

pub struct MoveAction {
    pub entity: Entity,
    pub direction: Direction,
}

impl Action for MoveAction {
    fn can_perform(&self, world: &mut ImmutableWorld) -> bool {
        let current_position = match world.get::<GridPosition>(self.entity) {
            Some(p) => p,
            None => return false,
        };

        let mut intended_position = current_position.clone();
        match self.direction {
            Direction::Up => intended_position.y += 1,
            Direction::Down => intended_position.y -= 1,
            Direction::Left => intended_position.x -= 1,
            Direction::Right => intended_position.x += 1,
        }

        for position in world.query::<&GridPosition>().iter(&world) {
            if position == &intended_position {
                return false;
            }
        }
        true
    }

    fn perform(&mut self, world: &mut World) -> ActionStatus {
        let current_position = match world.get::<GridPosition>(self.entity) {
            Some(p) => p,
            None => return ActionStatus::Finished,
        };

        let mut intended_position = current_position.clone();
        match self.direction {
            Direction::Up => intended_position.y += 1,
            Direction::Down => intended_position.y -= 1,
            Direction::Left => intended_position.x -= 1,
            Direction::Right => intended_position.x += 1,
        }

        for position in world.query::<&GridPosition>().iter(&world) {
            if position == &intended_position {
                return ActionStatus::Finished;
            }
        }

        *world.get_mut::<GridPosition>(self.entity).unwrap() = intended_position;
        world.add_action(MoveAnimationAction::new(self.entity, self.direction));
        ActionStatus::Finished
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct MoveAnimationAction {
    entity: Entity,
    direction: Direction,
    duration: Duration,
    pixels_left: f32,
}

impl MoveAnimationAction {
    fn new(entity: Entity, direction: Direction) -> Self {
        Self {
            entity,
            direction,
            duration: Duration::from_millis(80),
            pixels_left: 32.0,
        }
    }
}

impl Action for MoveAnimationAction {
    fn can_perform(&self, _: &mut ImmutableWorld) -> bool {
        true
    }

    fn perform(&mut self, world: &mut World) -> ActionStatus {
        let dt = world.get_resource::<Time>().unwrap().delta().as_secs_f32();
        let duration = self.duration.as_secs_f32();
        let pixels_to_move = ((dt / duration) * 32.0).min(self.pixels_left);

        // TODO: Use "&mut transform.into_inner().translation" from bevy 0.6
        let mut transform = match world.get_mut::<Transform>(self.entity) {
            Some(t) => t,
            None => return ActionStatus::Finished,
        };
        let translation = &mut transform.translation;
        match self.direction {
            Direction::Up => translation.y += pixels_to_move,
            Direction::Down => translation.y -= pixels_to_move,
            Direction::Left => translation.x -= pixels_to_move,
            Direction::Right => translation.x += pixels_to_move,
        }

        self.pixels_left -= pixels_to_move;
        if self.pixels_left == 0.0 {
            ActionStatus::Finished
        } else {
            ActionStatus::Unfinished
        }
    }
}
