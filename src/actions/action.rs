use crate::world::ImmutableWorld;
use bevy::prelude::World;
use std::time::{Duration, Instant};

pub struct ActionStack(Vec<Box<dyn Action>>);

impl ActionStack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_not_empty(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn add(&mut self, action: Box<dyn Action>) {
        self.0.push(action);
    }
}

pub trait Action: Send + Sync {
    fn can_attempt(&self, world: &mut ImmutableWorld) -> bool;
    fn attempt(&mut self, world: &mut World) -> ActionStatus;

    fn to_brain_decision(self) -> Option<Box<dyn Action>>
    where
        Self: Sized + 'static,
    {
        Some(Box::new(self))
    }

    fn to_brain_decision_if_can_attempt(self, world: &mut ImmutableWorld) -> Option<Box<dyn Action>>
    where
        Self: Sized + 'static,
    {
        if self.can_attempt(world) {
            self.to_brain_decision()
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ActionStatus {
    Finished,
    Unfinished,
}

/// Take the top action from the stack and run it
/// Put the action back afterwards if it's unfinished
/// Repeat until out of actions, an action is unfinished, or it's been running for at least 8ms
pub fn attempt_next_action(world: &mut World) {
    let start = Instant::now();
    loop {
        let mut action_stack = world.get_resource_mut::<ActionStack>().unwrap();
        match action_stack.0.pop() {
            Some(mut action) => {
                let action_index = action_stack.0.len();
                let action_status = action.attempt(world);

                if action_status == ActionStatus::Unfinished {
                    let mut action_stack = world.get_resource_mut::<ActionStack>().unwrap();
                    action_stack.0.insert(action_index, action);
                    break;
                }

                if start.elapsed() >= Duration::from_millis(8) {
                    break;
                }
            }
            None => break,
        }
    }
}
