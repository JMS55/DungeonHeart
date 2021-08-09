use crate::actions::{Action, ActionStack};
use bevy::ecs::prelude::QueryState;
use bevy::ecs::query::{ReadOnlyFetch, WorldQuery};
use bevy::prelude::World;
use std::ops::Deref;

pub struct ImmutableWorld<'a> {
    world: &'a mut World,
}

impl<'a> ImmutableWorld<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Self { world }
    }

    pub fn query<Q>(&mut self) -> QueryState<Q, ()>
    where
        Q: WorldQuery,
        <Q as WorldQuery>::Fetch: ReadOnlyFetch,
    {
        self.world.query()
    }
}

impl Deref for ImmutableWorld<'_> {
    type Target = World;

    fn deref(&self) -> &World {
        &self.world
    }
}

pub trait WorldExt {
    fn add_action<T: Action + 'static>(&mut self, action: T);
}

impl WorldExt for World {
    fn add_action<T: Action + 'static>(&mut self, action: T) {
        self.get_resource_mut::<ActionStack>()
            .unwrap()
            .add(Box::new(action));
    }
}
