use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct Item;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct Consumable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum EquipmentSlot {
    Head,
    Torso,
    Legs,
    Feet,
    Hands,
    MainHand,
    OffHand,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}
