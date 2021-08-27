use crate::actions::{Action, ActionStatus};
use crate::components::Direction;
use crate::world::{ImmutableWorld, WorldExt};
use bevy::core::Time;
use bevy::math::{IVec2, Rect};
use bevy::prelude::{Entity, GlobalTransform, Transform, World};
use std::time::Duration;

pub struct MoveAction {
    pub entity: Entity,
    pub direction: Direction,
}

impl MoveAction {
    fn intended_position(&self, world: &mut ImmutableWorld) -> Option<IVec2> {
        let current_position = world.get::<IVec2>(self.entity)?;
        let intended_position = *current_position + self.direction.as_offset();

        if world
            .query::<&IVec2>()
            .iter(&world)
            .any(|position| position == &intended_position)
        {
            None
        } else {
            Some(intended_position)
        }
    }
}

impl Action for MoveAction {
    fn can_attempt(&self, world: &mut ImmutableWorld) -> bool {
        self.intended_position(world).is_some()
    }

    fn attempt(&mut self, world: &mut World) -> ActionStatus {
        if let Some(intended_position) = self.intended_position(&mut ImmutableWorld::new(world)) {
            *world.get_mut::<IVec2>(self.entity).unwrap() = intended_position;
            world.add_action(MoveAnimationAction::new(self.entity, self.direction));
        }

        ActionStatus::Finished
    }
}

struct MoveAnimationAction {
    entity: Entity,
    direction: Direction,
    duration: Duration,
    pixels_remaining: f32,
}

impl MoveAnimationAction {
    fn new(entity: Entity, direction: Direction) -> Self {
        Self {
            entity,
            direction,
            duration: Duration::from_millis(40),
            pixels_remaining: 32.0,
        }
    }
}

impl Action for MoveAnimationAction {
    fn can_attempt(&self, _: &mut ImmutableWorld) -> bool {
        unreachable!()
    }

    // TODO: Use "&/&mut transform.into_inner().translation" from bevy 0.6
    fn attempt(&mut self, world: &mut World) -> ActionStatus {
        let transform = match world.get::<GlobalTransform>(self.entity) {
            Some(t) => t,
            None => return ActionStatus::Finished,
        };
        let translation = &transform.translation;
        let mut animation_rect = Rect {
            left: translation.x - 16.0,
            right: translation.x + 16.0,
            top: translation.y + 16.0,
            bottom: translation.y - 16.0,
        };
        match self.direction {
            Direction::Up => animation_rect.top += self.pixels_remaining,
            Direction::Down => animation_rect.bottom -= self.pixels_remaining,
            Direction::Left => animation_rect.left -= self.pixels_remaining,
            Direction::Right => animation_rect.right += self.pixels_remaining,
        }

        if world.is_rect_visible(animation_rect) {
            let dt = world.get_resource::<Time>().unwrap().delta().as_secs_f32();
            let duration = self.duration.as_secs_f32();
            let pixels_to_move = ((dt / duration) * 32.0).min(self.pixels_remaining);

            let mut transform = match world.get_mut::<Transform>(self.entity) {
                Some(t) => t,
                None => return ActionStatus::Finished,
            };
            transform.translation +=
                self.direction.as_offset().as_f32().extend(0.0) * pixels_to_move;

            self.pixels_remaining -= pixels_to_move;
            if self.pixels_remaining == 0.0 {
                ActionStatus::Finished
            } else {
                ActionStatus::Unfinished
            }
        } else {
            let mut transform = match world.get_mut::<Transform>(self.entity) {
                Some(t) => t,
                None => return ActionStatus::Finished,
            };
            transform.translation +=
                self.direction.as_offset().as_f32().extend(0.0) * self.pixels_remaining;

            ActionStatus::Finished
        }
    }
}
