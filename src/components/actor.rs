use crate::actions::{Action, ActionStack};
use crate::world::ImmutableWorld;
use bevy::prelude::{Entity, Query, ResMut, World};
use dyn_clone::{clone_trait_object, DynClone};

pub struct Actor {
    pub brain: Box<dyn Brain>,
    pub turn_group: TurnGroup,
    ready_to_act: bool,
}

impl Actor {
    pub fn new<B: Brain + 'static>(brain: B, turn_group: TurnGroup) -> Self {
        Self {
            brain: Box::new(brain),
            turn_group,
            ready_to_act: false,
        }
    }
}

clone_trait_object!(Brain);
pub trait Brain: DynClone + Send + Sync {
    fn decide_action(
        &mut self,
        this_entity: Entity,
        world: &mut ImmutableWorld,
    ) -> Option<Box<dyn Action>>;
}

impl<F> Brain for F
where
    F: (Fn(Entity, &mut ImmutableWorld) -> Option<Box<dyn Action>>) + DynClone + Send + Sync,
{
    fn decide_action(
        &mut self,
        this_entity: Entity,
        world: &mut ImmutableWorld,
    ) -> Option<Box<dyn Action>> {
        (self)(this_entity, world)
    }
}

#[derive(PartialEq, Eq)]
pub enum TurnGroup {
    Player,
    Enemy,
    Neutral,
}

/// If no more actors left for current turn group
/// Advance to next turn group
/// Ready all actors in the new group
pub fn determine_turn_group(mut turn_group: ResMut<TurnGroup>, mut actors: Query<&mut Actor>) {
    let actors = actors.iter_mut().collect::<Vec<_>>();
    let actors_left_for_turn = actors
        .iter()
        .filter(|actor| actor.turn_group == *turn_group && actor.ready_to_act)
        .count();

    if actors_left_for_turn == 0 {
        *turn_group = match *turn_group {
            TurnGroup::Player => TurnGroup::Enemy,
            TurnGroup::Enemy => TurnGroup::Neutral,
            TurnGroup::Neutral => TurnGroup::Player,
        };

        for mut actor in actors {
            if actor.turn_group == *turn_group {
                actor.ready_to_act = true;
            }
        }
    }
}

/// If action stack is empty
/// Asks actors in current turn group for an action if they are ready to act
/// Stops on the first action being given
/// If the current turn group is Players, keep asking the first actor each tick until they give one
/// If the current turn group isn't Players, ask each actor 3 times a tick
pub fn decide_next_action(world: &mut World) {
    if !world.get_resource::<ActionStack>().unwrap().is_empty() {
        return;
    }

    if world.get_resource::<TurnGroup>().unwrap() == &TurnGroup::Player {
        let actor_entity = world
            .query::<(&Actor, Entity)>()
            .iter_mut(world)
            .filter(|(actor, _)| actor.ready_to_act)
            .map(|(_, actor_entity)| actor_entity)
            .next();
        if let Some(actor_entity) = actor_entity {
            let mut brain_clone = world.get::<Actor>(actor_entity).unwrap().brain.clone();
            let action = brain_clone.decide_action(actor_entity, &mut ImmutableWorld::new(world));
            world.get_mut::<Actor>(actor_entity).unwrap().brain = brain_clone;

            if let Some(action) = action {
                world.get_mut::<Actor>(actor_entity).unwrap().ready_to_act = false;
                world.get_resource_mut::<ActionStack>().unwrap().add(action);
            }
        }
    } else {
        let actor_entities = world
            .query::<(&Actor, Entity)>()
            .iter_mut(world)
            .filter(|(actor, _)| actor.ready_to_act)
            .map(|(_, actor_entity)| actor_entity)
            .collect::<Vec<_>>();
        for decision_attempt in 1..=3 {
            for actor_entity in actor_entities.iter().copied() {
                let mut brain_clone = world.get::<Actor>(actor_entity).unwrap().brain.clone();
                let action =
                    brain_clone.decide_action(actor_entity, &mut ImmutableWorld::new(world));
                world.get_mut::<Actor>(actor_entity).unwrap().brain = brain_clone;

                if let Some(action) = action {
                    world.get_mut::<Actor>(actor_entity).unwrap().ready_to_act = false;
                    world.get_resource_mut::<ActionStack>().unwrap().add(action);
                    return;
                } else if decision_attempt == 3 {
                    world.get_mut::<Actor>(actor_entity).unwrap().ready_to_act = false;
                }
            }
        }
    }
}
