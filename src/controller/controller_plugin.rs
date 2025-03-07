use bevy::prelude::*;

use crate::controller::event_systems::*;
use crate::controller::events::PlayerAction;
use crate::controller::systems::*;
use crate::AppSet;
use crate::RunningState;

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<PlayerAction>();

        // Register systems
        app.add_systems(
            Update,
            keyboard_input
                .in_set(AppSet::RecordInput)
                .run_if(in_state(RunningState::Running)),
        )
        // .add_systems(Update, handle_player_actions.in_set(AppSet::Update))
        .add_observer(handle_player_actions);
    }
}
