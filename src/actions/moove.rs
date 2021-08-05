use crate::actions::{Action, ActionStatus};
use crate::components::GridPosition;
use crate::immutable_world::ImmutableWorld;
use bevy::prelude::{Entity, Transform, World};

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
        world
            .get_mut::<Transform>(self.entity)
            .unwrap()
            .translation
            .x = (intended_position.x * 32) as f32;
        world
            .get_mut::<Transform>(self.entity)
            .unwrap()
            .translation
            .y = (intended_position.y * 32) as f32;
        *world.get_mut::<GridPosition>(self.entity).unwrap() = intended_position;
        ActionStatus::Finished
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
