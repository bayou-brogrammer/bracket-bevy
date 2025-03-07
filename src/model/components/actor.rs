use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct Monster;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct NPC;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Stats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

impl Stats {
    pub fn new(health: i32, attack: i32, defense: i32, speed: i32) -> Self {
        Self {
            health,
            max_health: health,
            attack,
            defense,
            speed,
        }
    }
}
