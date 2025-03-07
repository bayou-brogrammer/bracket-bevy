use bevy::prelude::*;
use bracket_lib::bevy::BTermBuilder;

use super::systems::render_system;
use crate::AppSet;
use crate::RunningState;

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BTermBuilder::simple_80x50()).add_systems(
            Update,
            render_system
                .run_if(not(in_state(RunningState::Paused)))
                .in_set(AppSet::Render),
        );
    }
}
