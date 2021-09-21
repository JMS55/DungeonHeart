use crate::actions::{Action, ActionStack};
use crate::world::ImmutableWorld;
use bevy::prelude::{Entity, Query, ResMut, World};

pub struct Actor {
    pub brain: Box<dyn Brain>,
    pub turn_group: TurnGroup,
    ready_to_act: bool,
    unlimited_decision_attempts: bool,
}

impl Actor {
    pub fn new<B: Brain + 'static>(brain: B, turn_group: TurnGroup) -> Self {
        Self {
            brain: Box::new(brain),
            turn_group,
            ready_to_act: false,
            unlimited_decision_attempts: false,
        }
    }

    pub fn new_unlimited_decision_attempts<B: Brain + 'static>(
        brain: B,
        turn_group: TurnGroup,
    ) -> Self {
        Self {
            brain: Box::new(brain),
            turn_group,
            ready_to_act: false,
            unlimited_decision_attempts: true,
        }
    }
}

pub trait Brain: Send + Sync {
    fn decide_action(
        &mut self,
        this_entity: Entity,
        this_turn_group: TurnGroup,
        world: &mut ImmutableWorld,
    ) -> Option<Box<dyn Action>>;
}

impl<F> Brain for F
where
    F: (Fn(Entity, TurnGroup, &mut ImmutableWorld) -> Option<Box<dyn Action>>) + Send + Sync,
{
    fn decide_action(
        &mut self,
        this_entity: Entity,
        this_turn_group: TurnGroup,
        world: &mut ImmutableWorld,
    ) -> Option<Box<dyn Action>> {
        (self)(this_entity, this_turn_group, world)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TurnGroup {
    Player,
    Enemy,
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
            TurnGroup::Enemy => TurnGroup::Player,
        };

        for mut actor in actors {
            if actor.turn_group == *turn_group {
                actor.ready_to_act = true;
            }
        }
    }
}

/// If action stack is empty
/// Ask each actor ready to act in current turn group for an action
/// Stop when an actor gives an action
///
/// When an actor fails to give an action:
/// If actor.unlimited_decision_attempts, keep asking the same actor over multiple ticks until they give one
/// Else move on to the next actor and loop back around, max 3 attempts per actor
pub fn decide_next_action(world: &mut World) {
    if world.get_resource::<ActionStack>().unwrap().is_not_empty() {
        return;
    }

    let current_turn_group = *world.get_resource::<TurnGroup>().unwrap();
    let actor_entities = world
        .query::<(&Actor, Entity)>()
        .iter_mut(world)
        .filter(|(actor, _)| actor.turn_group == current_turn_group && actor.ready_to_act)
        .map(|(_, actor_entity)| actor_entity)
        .collect::<Vec<_>>();

    for decision_attempt in 1..=3 {
        for actor_entity in actor_entities.iter().copied() {
            let mut actor = world.entity_mut(actor_entity).remove::<Actor>().unwrap();
            let turn_group = actor.turn_group;
            let unlimited_decision_attempts = actor.unlimited_decision_attempts;

            let action = actor.brain.decide_action(
                actor_entity,
                turn_group,
                &mut ImmutableWorld::new(world),
            );

            if action.is_some() || (decision_attempt == 3 && !unlimited_decision_attempts) {
                actor.ready_to_act = false;
            }
            world.entity_mut(actor_entity).insert(actor);

            if let Some(action) = action {
                world.get_resource_mut::<ActionStack>().unwrap().add(action);
                return;
            } else if unlimited_decision_attempts {
                return;
            }
        }
    }
}
