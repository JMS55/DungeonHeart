use crate::actions::{Action, ActionStatus};
use crate::components::Health;
use crate::world::{ImmutableWorld, WorldExt};
use bevy::core::Time;
use bevy::math::Rect;
use bevy::prelude::{Entity, GlobalTransform, World};
use std::time::Duration;

pub struct DamageAction {
    pub damage: u32,
    pub target: Entity,
}

impl Action for DamageAction {
    fn can_attempt(&self, world: &mut ImmutableWorld) -> bool {
        world.get::<Health>(self.target).is_some()
    }

    fn attempt(&mut self, world: &mut World) -> ActionStatus {
        if let Some(target_health) = &mut world.get_mut::<Health>(self.target) {
            target_health.current = target_health.current.saturating_sub(self.damage);

            if target_health.current == 0 {
                world.add_action(DeleteAction {
                    entity: self.target,
                });
            }
            world.add_action(DamageAnimationAction::new(self.target));
        }

        ActionStatus::Finished
    }
}

struct DamageAnimationAction {
    entity: Entity,
    duration: Duration,
    time_remaining: Duration,
}

impl DamageAnimationAction {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            duration: Duration::from_secs(1),
            time_remaining: Duration::from_secs(1),
        }
    }
}

impl Action for DamageAnimationAction {
    fn can_attempt(&self, _: &mut ImmutableWorld) -> bool {
        unreachable!()
    }

    fn attempt(&mut self, world: &mut World) -> ActionStatus {
        let transform = match world.get::<GlobalTransform>(self.entity) {
            Some(t) => t,
            None => return ActionStatus::Finished,
        };
        let translation = &transform.translation;
        let animation_rect = Rect {
            left: translation.x - 16.0,
            right: translation.x + 16.0,
            top: translation.y + 16.0,
            bottom: translation.y - 16.0,
        };

        if world.is_rect_visible(animation_rect) {
            let dt = world.get_resource::<Time>().unwrap().delta();
            // TODO: Animate opacity flashing
            self.time_remaining = self.time_remaining.saturating_sub(dt);
            if !self.time_remaining.is_zero() {
                return ActionStatus::Unfinished;
            }
        }

        ActionStatus::Finished
    }
}

struct DeleteAction {
    entity: Entity,
}

impl Action for DeleteAction {
    fn can_attempt(&self, _: &mut ImmutableWorld) -> bool {
        unreachable!()
    }

    fn attempt(&mut self, world: &mut World) -> ActionStatus {
        world.despawn(self.entity);
        ActionStatus::Finished
    }
}
