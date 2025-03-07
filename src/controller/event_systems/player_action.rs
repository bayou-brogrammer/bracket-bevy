use bevy::prelude::*;

use crate::controller::events::PlayerAction;
use crate::model::components::{FieldOfView, Player, Position};
use crate::model::resources::{GameLog, Map, TurnState};
use crate::model::systems::try_move_entity;

pub fn handle_player_actions(
    player_action_trigger: Trigger<PlayerAction>,
    player_query: Query<Entity, With<Player>>,
    map: Res<Map>,
    mut game_log: ResMut<GameLog>,
    mut turn_state: ResMut<TurnState>,
    mut position_query: Query<&mut Position>,
    mut fov_query: Query<&mut FieldOfView>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    // You can access the trigger data via the `Observer`
    match player_action_trigger.event() {
        PlayerAction::Move(direction) => {
            let moved = try_move_entity(
                player_entity,
                *direction,
                &map,
                &mut position_query,
                &mut fov_query,
            );

            if moved {
                game_log.add_entry_srgb("You move.", Color::srgb(0.5, 0.5, 1.0));
            } else {
                game_log.add_entry_srgb("Something blocks your way.", Color::srgb(1.0, 0.3, 0.3));
            }
        }
        PlayerAction::Wait => {
            game_log.add_entry_srgb("You wait.", Color::srgb(0.5, 0.5, 1.0));
        }
        PlayerAction::PickupItem => {
            // Get player position
            if let Ok(_player_pos) = position_query.get(player_entity) {
                // Find items at player position
                // This would be implemented in a more complete game
                game_log.add_entry_srgb("You pick up an item.", Color::srgb(0.0, 1.0, 0.0));
            }
        }
        PlayerAction::UseItem(_item_entity) => {
            // Use item logic would go here
            game_log.add_entry_srgb("You use an item.", Color::srgb(0.0, 1.0, 0.0));
        }
        PlayerAction::DropItem(_item_entity) => {
            // Drop item logic would go here
            game_log.add_entry_srgb("You drop an item.", Color::srgb(0.5, 0.5, 0.5));
        }
        PlayerAction::EquipItem(_item_entity) => {
            // Equip item logic would go here
            game_log.add_entry_srgb("You equip an item.", Color::srgb(0.0, 1.0, 0.0));
        }
        PlayerAction::UnequipItem(_item_entity) => {
            // Unequip item logic would go here
            game_log.add_entry_srgb("You unequip an item.", Color::srgb(0.5, 0.5, 0.5));
        }
    }

    // After player action, update turn state
    turn_state.next_turn();
}
