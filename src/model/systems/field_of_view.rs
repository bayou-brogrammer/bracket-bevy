use bevy::prelude::*;
use bracket_lib::prelude::{field_of_view, Point};

use crate::model::components::{FieldOfView, Position};
use crate::model::resources::Map;

pub fn update_fov(mut map: ResMut<Map>, mut query: Query<(&Position, &mut FieldOfView)>) {
    for (position, mut fov) in query.iter_mut() {
        if fov.is_dirty {
            fov.visible_tiles.clear();

            // Use bracket-lib's field_of_view function
            let center = Point::new(position.x, position.y);
            let fov_tiles = field_of_view(center, fov.range, &*map);

            // Convert to our Position type and add to visible_tiles
            for tile in fov_tiles.iter() {
                let pos = Position::new(tile.x, tile.y);
                fov.visible_tiles.insert(pos);
            }

            // Update map visibility
            map.set_visibility(&fov.visible_tiles);

            fov.is_dirty = false;
        }
    }
}

// Implement the field_of_view trait for Map
impl bracket_lib::prelude::BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        if idx >= self.tiles.len() {
            return true;
        }
        self.tiles[idx].is_opaque()
    }

    fn get_available_exits(
        &self,
        idx: usize,
    ) -> bracket_lib::prelude::SmallVec<[(usize, f32); 10]> {
        let mut exits = bracket_lib::prelude::SmallVec::new();
        let (x, y) = self.idx_xy(idx);

        // Check cardinal directions
        let directions = [
            (x, y - 1), // North
            (x + 1, y), // East
            (x, y + 1), // South
            (x - 1, y), // West
        ];

        for (nx, ny) in directions.iter() {
            if self.in_bounds(*nx, *ny) && !self.is_blocked(*nx, *ny) {
                let new_idx = self.xy_idx(*nx, *ny);
                exits.push((new_idx, 1.0));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let (x1, y1) = self.idx_xy(idx1);
        let (x2, y2) = self.idx_xy(idx2);
        let dx = (x1 - x2).abs() as f32;
        let dy = (y1 - y2).abs() as f32;

        // Euclidean distance
        (dx * dx + dy * dy).sqrt()
    }
}

// Implement the Algorithm2D trait for Map
impl bracket_lib::prelude::Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point.x, point.y)
    }

    fn point2d_to_index(&self, point: Point) -> usize {
        self.xy_idx(point.x, point.y)
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let (x, y) = self.idx_xy(idx);
        Point::new(x, y)
    }
}
