use bevy::prelude::*;

use crate::model::systems::MoveDirection;

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Move(MoveDirection),
    Wait,
    PickupItem,
    UseItem(Entity),
    DropItem(Entity),
    EquipItem(Entity),
    UnequipItem(Entity),
}
