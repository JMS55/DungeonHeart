use crate::actions::{Action, ActionStatus};
use crate::bundles::{Floor, Wall};
use crate::components::KeepBetweenFloors;
use crate::world::ImmutableWorld;
use bevy::math::{ivec2, IVec2};
use bevy::prelude::{Entity, Without, World};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::HashSet;

// TODO: Make sure no rooms are inaccessible
// TODO: Spawn entities in batches

pub struct RegenerateDungeonAction {
    wall_positions: HashSet<IVec2>,
    floor_positions: HashSet<IVec2>,
}

impl RegenerateDungeonAction {
    pub fn new() -> Self {
        Self {
            wall_positions: HashSet::new(),
            floor_positions: HashSet::new(),
        }
    }
}

impl Action for RegenerateDungeonAction {
    fn can_attempt(&self, _: &mut ImmutableWorld) -> bool {
        unreachable!()
    }

    fn attempt(&mut self, world: &mut World) -> ActionStatus {
        Self::cleanup_previous(world);
        self.plan_rooms(world);
        self.plan_corridors(world);
        self.create_walls(world);
        self.create_floors(world);

        ActionStatus::Finished
    }
}

impl RegenerateDungeonAction {
    fn cleanup_previous(world: &mut World) {
        world.get_resource_mut::<Vec<Room>>().unwrap().clear();

        let entities_to_delete = world
            .query_filtered::<Entity, Without<KeepBetweenFloors>>()
            .iter(world)
            .collect::<Vec<_>>();
        for entity in entities_to_delete {
            world.despawn(entity);
        }
    }

    fn plan_rooms(&mut self, world: &mut World) {
        let mut rooms = world.get_resource_mut::<Vec<Room>>().unwrap();
        let starting_room = Room {
            center: ivec2(0, 0),
            radius: ivec2(3, 3),
        };
        rooms.push(starting_room);

        let mut rng = rand::thread_rng();
        'room_placing_loop: for _ in 0..200 {
            let room = Room {
                center: ivec2(rng.gen_range(-30..31), rng.gen_range(-30..31)),
                radius: ivec2(rng.gen_range(2..8), rng.gen_range(2..8)),
            };
            for other_room in rooms.iter() {
                let required_gap = rng.gen_range(3..10);
                let gap = ((room.center - other_room.center).abs()
                    - room.radius
                    - other_room.radius
                    - IVec2::splat(3))
                .max_element();
                if gap < required_gap && gap != -1 {
                    continue 'room_placing_loop;
                }
            }
            rooms.push(room);
        }
    }

    fn plan_corridors(&mut self, world: &mut World) {
        let rooms = world.get_resource::<Vec<Room>>().unwrap();
        let mut rng = rand::thread_rng();

        for (start_room_index, start_room) in rooms.iter().enumerate() {
            let mut end_room_index = rng.gen_range(0..rooms.len());
            while end_room_index == start_room_index {
                end_room_index = rng.gen_range(0..rooms.len());
            }
            let end_room = &rooms[end_room_index];

            let start = start_room.random_tile_inside(&mut rng);
            let end = end_room.random_tile_inside(&mut rng);
            let xx = ivec2(start.x, end.x);
            let yy = ivec2(start.y, end.y);

            for x in xx.min_element()..xx.max_element() {
                self.floor_positions.insert(ivec2(x, start.y));
            }
            for y in yy.min_element()..=yy.max_element() {
                self.floor_positions.insert(ivec2(end.x, y));
            }
        }
    }

    fn create_walls(&mut self, world: &mut World) {
        let rooms = world.get_resource::<Vec<Room>>().unwrap();

        for room in rooms {
            for x in -(room.radius.x + 1)..=(room.radius.x + 1) {
                self.wall_positions
                    .insert(ivec2(room.center.x + x, room.center.y + room.radius.y + 1));
                self.wall_positions
                    .insert(ivec2(room.center.x + x, room.center.y - room.radius.y - 1));
            }
            for y in -room.radius.y..=room.radius.y {
                self.wall_positions
                    .insert(ivec2(room.center.x + room.radius.x + 1, room.center.y + y));
                self.wall_positions
                    .insert(ivec2(room.center.x - room.radius.x - 1, room.center.y + y));
            }
        }

        for corridor_position in &self.floor_positions {
            'neighbor_loop: for neighbor in &neighbors(corridor_position) {
                for room in rooms {
                    let x_range =
                        (room.center.x - room.radius.x - 1)..=(room.center.x + room.radius.x + 1);
                    let y_range =
                        (room.center.y - room.radius.y - 1)..=(room.center.y + room.radius.y + 1);
                    if x_range.contains(&neighbor.x) && y_range.contains(&neighbor.y) {
                        continue 'neighbor_loop;
                    }
                }
                self.wall_positions.insert(ivec2(neighbor.x, neighbor.y));
            }
        }

        self.wall_positions = self
            .wall_positions
            .difference(&self.floor_positions)
            .copied()
            .collect();
        for position in &self.wall_positions {
            let variant = self.wall_positions.contains(&(*position - ivec2(0, 1)));
            world
                .spawn()
                .insert_bundle(Wall::new(position.x, position.y, !variant));
        }
    }

    fn create_floors(&mut self, world: &mut World) {
        let rooms = world.get_resource::<Vec<Room>>().unwrap();
        for room in rooms {
            for x in -room.radius.x..=room.radius.x {
                for y in -room.radius.y..=room.radius.y {
                    self.floor_positions
                        .insert(ivec2(room.center.x + x, room.center.y + y));
                }
            }
        }

        for position in &self.floor_positions {
            world
                .spawn()
                .insert_bundle(Floor::new(position.x, position.y));
        }
    }
}

pub struct Room {
    center: IVec2,
    radius: IVec2,
}

impl Room {
    pub fn random_tile_inside(&self, rng: &mut ThreadRng) -> IVec2 {
        let n1 = self.center - self.radius;
        let n2 = self.center + self.radius;
        ivec2(rng.gen_range(n1.x..=n2.x), rng.gen_range(n1.y..=n2.y))
    }
}

fn neighbors(p: &IVec2) -> [IVec2; 8] {
    [
        *p + ivec2(-1, 1),
        *p + ivec2(0, 1),
        *p + ivec2(1, 1),
        *p + ivec2(1, 0),
        *p + ivec2(1, -1),
        *p + ivec2(0, -1),
        *p + ivec2(-1, -1),
        *p + ivec2(-1, 0),
    ]
}
