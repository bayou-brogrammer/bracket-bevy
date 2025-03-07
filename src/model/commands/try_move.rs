use bevy::{ecs::system::SystemState, prelude::*};

use crate::model::{
    components::{Position, TerrainType},
    resources::{GameLog, Map},
    systems::MoveDirection,
};

pub struct TryMove(pub MoveDirection);

impl TryMove {
    pub fn new(dir: MoveDirection) -> Self {
        Self(dir)
    }
}

impl EntityCommand for TryMove {
    fn apply(self, entity: Entity, world: &mut World) {
        println!("TryMove command applied");
        let mut state: SystemState<(Res<Map>, ResMut<GameLog>, Query<&mut Position>)> =
            SystemState::new(world);

        let (map, mut game_log, mut position_query) = state.get_mut(world);

        // Get the entity's current position
        let Ok(mut position) = position_query.get_mut(entity) else {
            game_log.add_entry("Entity cannot move (no position component).");
            return;
        };

        // Calculate the new position
        let new_position = *position + self.0.delta();

        // Check if the new position is valid
        let terrain_type = map.get_tile(new_position.x, new_position.y);

        // Process movement based on terrain type
        match terrain_type {
            TerrainType::Floor => {
                game_log.add_entry(format!("You move {:?}", self.0));
                *position = new_position;
            }
            TerrainType::Wall => {
                game_log.add_entry("You bump into a wall.");
            }
            TerrainType::Stairs => {
                game_log.add_entry("You move to the next level.");
                *position = new_position;
                // You might want to trigger level transition here or in a separate system
            }
        }
    }
}
