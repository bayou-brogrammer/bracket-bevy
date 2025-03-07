use bevy::prelude::*;
use bracket_lib::prelude::RandomNumberGenerator;

use crate::model::components::TerrainType;
use crate::model::resources::Map;
use crate::RunningState;

#[derive(Debug, Clone)]
pub struct Room {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn intersect(&self, other: &Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub fn generate_map(mut map: ResMut<Map>, mut next_state: ResMut<NextState<RunningState>>) {
    println!("Generating map");

    let mut rng = RandomNumberGenerator::new();
    let room_max_size = 10;
    let room_min_size = 6;
    let max_rooms = 30;

    let mut rooms: Vec<Room> = Vec::new();

    for _ in 0..max_rooms {
        let w = rng.range(room_min_size, room_max_size);
        let h = rng.range(room_min_size, room_max_size);
        let x = rng.range(1, map.width - w - 1);
        let y = rng.range(1, map.height - h - 1);

        let new_room = Room::new(x, y, w, h);

        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false;
                break;
            }
        }

        if ok {
            apply_room_to_map(&mut map, &new_room);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    // Add stairs in the last room
    if let Some(last_room) = rooms.last() {
        let (x, y) = last_room.center();
        let idx = map.xy_idx(x, y);
        map.tiles[idx] = TerrainType::Stairs;
    }

    // Transition to Running state
    next_state.set(RunningState::Running);
}

fn apply_room_to_map(map: &mut Map, room: &Room) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TerrainType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in i32::min(x1, x2)..=i32::max(x1, x2) {
        let idx = map.xy_idx(x, y);
        if idx < map.tiles.len() {
            map.tiles[idx] = TerrainType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in i32::min(y1, y2)..=i32::max(y1, y2) {
        let idx = map.xy_idx(x, y);
        if idx < map.tiles.len() {
            map.tiles[idx] = TerrainType::Floor;
        }
    }
}
