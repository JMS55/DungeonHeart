use crate::actions::{Action, ActionStatus};
use crate::bundles::{Floor, Wall};
use crate::components::KeepBetweenFloors;
use crate::world::ImmutableWorld;
use bevy::math::IVec2;
use bevy::prelude::{Entity, Without, World};
use rand::Rng;
use std::collections::HashSet;

// TODO: Make sure no rooms are inaccessible
// TODO: Spawn entities in batches

pub struct RegenerateDungeonAction {
    rooms: Vec<Room>,
    wall_positions: HashSet<IVec2>,
    floor_positions: HashSet<IVec2>,
}

impl RegenerateDungeonAction {
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            wall_positions: HashSet::new(),
            floor_positions: HashSet::new(),
        }
    }
}

impl Action for RegenerateDungeonAction {
    fn can_attempt(&self, _: &mut ImmutableWorld) -> bool {
        true
    }

    fn attempt(&mut self, world: &mut World) -> ActionStatus {
        Self::cleanup_previous(world);
        self.plan_rooms();
        self.plan_corridors();
        self.create_walls(world);
        self.create_floors(world);

        ActionStatus::Finished
    }
}

impl RegenerateDungeonAction {
    fn cleanup_previous(world: &mut World) {
        let entities_to_delete = world
            .query_filtered::<Entity, Without<KeepBetweenFloors>>()
            .iter(world)
            .collect::<Vec<_>>();
        for entity in entities_to_delete {
            world.despawn(entity);
        }
    }

    fn plan_rooms(&mut self) {
        let starting_room = Room {
            center: IVec2::new(0, 0),
            radius: IVec2::new(3, 3),
        };
        self.rooms.push(starting_room);

        let mut rng = rand::thread_rng();
        'room_placing_loop: for _ in 0..200 {
            let room = Room {
                center: IVec2::new(rng.gen_range(-30..31), rng.gen_range(-30..31)),
                radius: IVec2::new(rng.gen_range(2..8), rng.gen_range(2..8)),
            };
            for other_room in &self.rooms {
                let required_gap = rng.gen_range(3..10);
                let x_gap = (room.center.x - other_room.center.x).abs()
                    - room.radius.x
                    - other_room.radius.x
                    - 3;
                let y_gap = (room.center.y - other_room.center.y).abs()
                    - room.radius.y
                    - other_room.radius.y
                    - 3;
                let actual_gap = x_gap.max(y_gap);
                if actual_gap < required_gap && actual_gap != -1 {
                    continue 'room_placing_loop;
                }
            }
            self.rooms.push(room);
        }
    }

    fn plan_corridors(&mut self) {
        let mut rng = rand::thread_rng();
        for (start_room_index, start_room) in self.rooms.iter().enumerate() {
            let mut end_room_index = rng.gen_range(0..self.rooms.len());
            while end_room_index == start_room_index {
                end_room_index = rng.gen_range(0..self.rooms.len());
            }
            let end_room = &self.rooms[end_room_index];

            let start_x = rng.gen_range(
                (start_room.center.x - start_room.radius.x as i32)
                    ..(start_room.center.x + start_room.radius.x as i32 + 1),
            );
            let start_y = rng.gen_range(
                (start_room.center.y - start_room.radius.y as i32)
                    ..(start_room.center.y + start_room.radius.y as i32 + 1),
            );
            let end_x = rng.gen_range(
                (end_room.center.x - end_room.radius.x as i32)
                    ..(end_room.center.x + end_room.radius.x as i32 + 1),
            );
            let end_y = rng.gen_range(
                (end_room.center.y - end_room.radius.y as i32)
                    ..(end_room.center.y + end_room.radius.y as i32 + 1),
            );

            for x in start_x.min(end_x)..start_x.max(end_x) {
                self.floor_positions.insert(IVec2::new(x, start_y));
            }
            for y in start_y.min(end_y)..=start_y.max(end_y) {
                self.floor_positions.insert(IVec2::new(end_x, y));
            }
        }
    }

    fn create_walls(&mut self, world: &mut World) {
        for room in &self.rooms {
            for x in -(room.radius.x + 1)..=(room.radius.x + 1) {
                self.wall_positions.insert(IVec2::new(
                    room.center.x + x,
                    room.center.y + room.radius.y + 1,
                ));
                self.wall_positions.insert(IVec2::new(
                    room.center.x + x,
                    room.center.y - room.radius.y - 1,
                ));
            }
            for y in -room.radius.y..=room.radius.y {
                self.wall_positions.insert(IVec2::new(
                    room.center.x + room.radius.x + 1,
                    room.center.y + y,
                ));
                self.wall_positions.insert(IVec2::new(
                    room.center.x - room.radius.x - 1,
                    room.center.y + y,
                ));
            }
        }

        for corridor_position in &self.floor_positions {
            'neighbor_loop: for neighbor in &neighbors(corridor_position) {
                for room in &self.rooms {
                    let x_range =
                        (room.center.x - room.radius.x - 1)..=(room.center.x + room.radius.x + 1);
                    let y_range =
                        (room.center.y - room.radius.y - 1)..=(room.center.y + room.radius.y + 1);
                    if x_range.contains(&neighbor.x) && y_range.contains(&neighbor.y) {
                        continue 'neighbor_loop;
                    }
                }
                self.wall_positions
                    .insert(IVec2::new(neighbor.x, neighbor.y));
            }
        }

        self.wall_positions = self
            .wall_positions
            .difference(&self.floor_positions)
            .copied()
            .collect();
        for position in &self.wall_positions {
            let variant = self
                .wall_positions
                .contains(&(*position - IVec2::new(0, 1)));
            world
                .spawn()
                .insert_bundle(Wall::new(position.x, position.y, !variant));
        }
    }

    fn create_floors(&mut self, world: &mut World) {
        for room in &self.rooms {
            for x in -room.radius.x..=room.radius.x {
                for y in -room.radius.y..=room.radius.y {
                    self.floor_positions
                        .insert(IVec2::new(room.center.x + x, room.center.y + y));
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

struct Room {
    center: IVec2,
    radius: IVec2,
}

fn neighbors(p: &IVec2) -> [IVec2; 8] {
    [
        *p + IVec2::new(-1, 1),
        *p + IVec2::new(0, 1),
        *p + IVec2::new(1, 1),
        *p + IVec2::new(1, 0),
        *p + IVec2::new(1, -1),
        *p + IVec2::new(0, -1),
        *p + IVec2::new(-1, -1),
        *p + IVec2::new(-1, 0),
    ]
}
