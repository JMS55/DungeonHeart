use crate::actions::{Action, ActionStatus};
use crate::world::ImmutableWorld;
use bevy::prelude::{Entity, World};

pub struct PrintEntityAction {
    pub entity: Entity,
}

impl Action for PrintEntityAction {
    fn can_attempt(&self, _: &mut ImmutableWorld) -> bool {
        true
    }

    fn attempt(&mut self, _: &mut World) -> ActionStatus {
        println!("Entity {:?} is acting", self.entity);
        ActionStatus::Finished
    }
}
