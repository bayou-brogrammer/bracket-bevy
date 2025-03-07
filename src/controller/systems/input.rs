use bevy::prelude::*;

use crate::controller::events::PlayerAction;
use crate::model::systems::MoveDirection;

pub fn keyboard_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    // Movement
    if keyboard.just_pressed(KeyCode::KeyW) {
        commands.trigger(PlayerAction::Move(MoveDirection::North));
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        commands.trigger(PlayerAction::Move(MoveDirection::South));
    }
    if keyboard.just_pressed(KeyCode::KeyA) {
        commands.trigger(PlayerAction::Move(MoveDirection::West));
    }
    if keyboard.just_pressed(KeyCode::KeyD) {
        commands.trigger(PlayerAction::Move(MoveDirection::East));
    }

    // Wait
    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(PlayerAction::Wait);
    }

    // Pickup item
    if keyboard.just_pressed(KeyCode::KeyG) {
        commands.trigger(PlayerAction::PickupItem);
    }
}
