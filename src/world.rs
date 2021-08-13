use crate::actions::{Action, ActionStack};
use bevy::ecs::prelude::QueryState;
use bevy::ecs::query::{ReadOnlyFetch, WorldQuery};
use bevy::math::{Rect, Vec2};
use bevy::prelude::{GlobalTransform, World};
use bevy::render::camera::OrthographicProjection;
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
    fn is_rect_visible(&mut self, rect: Rect<f32>) -> bool;
}

impl WorldExt for World {
    fn add_action<T: Action + 'static>(&mut self, action: T) {
        self.get_resource_mut::<ActionStack>()
            .unwrap()
            .add(Box::new(action));
    }

    fn is_rect_visible(&mut self, rect: Rect<f32>) -> bool {
        let camera = self
            .query::<(&OrthographicProjection, &GlobalTransform)>()
            .iter(self)
            .next();
        match camera {
            Some((projection, transform)) => {
                let p1 = Vec2::new(
                    projection.left + transform.translation.x,
                    projection.top + transform.translation.y,
                );
                let p2 = Vec2::new(
                    projection.right + transform.translation.x,
                    projection.bottom + transform.translation.y,
                );
                let p3 = Vec2::new(rect.left, rect.top);
                let p4 = Vec2::new(rect.right, rect.bottom);
                p2.y <= p3.y && p1.y >= p4.y && p2.x >= p3.x && p1.x <= p4.x
            }
            None => false,
        }
    }
}
