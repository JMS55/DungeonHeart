use crate::actions::{Action, ActionStatus};
use crate::immutable_world::ImmutableWorld;
use bevy::prelude::{Entity, World};

pub struct PrintEntityAction {
    pub entity: Entity,
}

impl Action for PrintEntityAction {
    fn can_perform(&self, _: &mut ImmutableWorld) -> bool {
        true
    }

    fn perform(&mut self, _: &mut World) -> ActionStatus {
        println!("Entity {:?} is acting", self.entity);
        ActionStatus::Finished
    }
}
