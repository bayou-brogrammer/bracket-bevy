use bevy::prelude::*;

use crate::model::components::Description;

#[derive(Component, Reflect, Default, Clone, Debug, PartialEq)]
#[reflect(Component)]
#[require(Description)]
pub enum TerrainType {
    #[default]
    Floor,
    Wall,
    Stairs,
}

impl TerrainType {
    pub fn is_walkable(&self) -> bool {
        matches!(self, TerrainType::Floor | TerrainType::Stairs)
    }

    pub fn is_opaque(&self) -> bool {
        matches!(self, TerrainType::Wall)
    }
}
