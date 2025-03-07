use bevy::prelude::*;
use std::collections::HashSet;

use crate::model::components::{Position, TerrainType};
use crate::model::ModelConstants;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TerrainType>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Default for Map {
    fn default() -> Self {
        let width = ModelConstants::MAP_WIDTH as i32;
        let height = ModelConstants::MAP_HEIGHT as i32;
        let size = (width * height) as usize;

        let mut map = Self {
            tiles: vec![TerrainType::Wall; size],
            width,
            height,
            revealed_tiles: vec![false; size],
            visible_tiles: vec![false; size],
        };

        // Create a simple room in the middle
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = TerrainType::Floor;
            }
        }

        map
    }
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;

        Self {
            tiles: vec![TerrainType::Wall; size],
            width,
            height,
            revealed_tiles: vec![false; size],
            visible_tiles: vec![false; size],
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &TerrainType {
        let idx = self.xy_idx(x, y);
        &self.tiles[idx]
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = (idx as i32) % self.width;
        let y = (idx as i32) / self.width;
        (x, y)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return true;
        }
        let idx = self.xy_idx(x, y);
        !self.tiles[idx].is_walkable()
    }

    pub fn clear_visibility(&mut self) {
        for visible in &mut self.visible_tiles {
            *visible = false;
        }
    }

    pub fn set_visibility(&mut self, visible_positions: &HashSet<Position>) {
        self.clear_visibility();
        for pos in visible_positions {
            if self.in_bounds(pos.x, pos.y) {
                let idx = self.xy_idx(pos.x, pos.y);
                self.visible_tiles[idx] = true;
                self.revealed_tiles[idx] = true;
            }
        }
    }
}
