use bevy::prelude::*;
use std::collections::HashSet;

use super::position::Position;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Position>,
    pub range: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
            is_dirty: true,
        }
    }

    pub fn is_visible(&self, pos: &Position) -> bool {
        self.visible_tiles.contains(pos)
    }

    pub fn clear(&mut self) {
        self.visible_tiles.clear();
        self.is_dirty = true;
    }
}
