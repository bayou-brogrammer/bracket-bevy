use bevy::prelude::*;

use crate::model::components::*;
use crate::model::resources::*;
use crate::model::systems::*;
use crate::AppSet;
use crate::RunningState;

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        // Register components
        app.register_type::<Position>()
            .register_type::<MapLevel>()
            .register_type::<Renderable>()
            .register_type::<crate::model::components::Name>()
            .register_type::<Player>()
            .register_type::<Monster>()
            .register_type::<NPC>()
            .register_type::<Stats>()
            .register_type::<FieldOfView>()
            .register_type::<Item>()
            .register_type::<Consumable>()
            .register_type::<Equippable>()
            .register_type::<InBackpack>()
            .register_type::<Equipped>();

        // Register resources
        app.init_resource::<Map>()
            .init_resource::<TurnState>()
            .init_resource::<GameLog>();

        // Register systems
        app.add_systems(OnEnter(RunningState::Load), setup_game)
            .add_systems(Update, generate_map.run_if(in_state(RunningState::Load)))
            .add_systems(
                OnEnter(RunningState::Running),
                (spawn_initial_player, spawn_monsters, spawn_items),
            )
            .add_systems(
                Update,
                update_fov
                    .in_set(AppSet::Visibility)
                    .run_if(in_state(RunningState::Running)),
            );
    }
}
