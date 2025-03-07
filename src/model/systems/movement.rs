use bevy::prelude::*;

use crate::model::components::{FieldOfView, Position};
use crate::model::resources::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveDirection {
    North,
    South,
    East,
    West,
}

impl MoveDirection {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            MoveDirection::North => (0, -1),
            MoveDirection::South => (0, 1),
            MoveDirection::East => (1, 0),
            MoveDirection::West => (-1, 0),
        }
    }
}

pub fn try_move_entity(
    entity: Entity,
    direction: MoveDirection,
    map: &Map,
    position_query: &mut Query<&mut Position>,
    fov_query: &mut Query<&mut FieldOfView>,
) -> bool {
    println!("try_move_entity fn");
    if let Ok(mut pos) = position_query.get_mut(entity) {
        let (dx, dy) = direction.delta();
        let new_x = pos.x + dx;
        let new_y = pos.y + dy;

        if !map.is_blocked(new_x, new_y) {
            pos.x = new_x;
            pos.y = new_y;

            // Mark FOV as dirty if entity has one
            if let Ok(mut fov) = fov_query.get_mut(entity) {
                fov.is_dirty = true;
            }

            return true;
        }
    }
    false
}
