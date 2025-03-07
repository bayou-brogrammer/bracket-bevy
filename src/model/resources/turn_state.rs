use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Reflect, Default)]
#[reflect(Resource)]
pub struct TurnState {
    pub current_entity: Option<Entity>,
    pub action_points: i32,
    pub turn_number: u32,
}

impl TurnState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_turn(&mut self) {
        self.turn_number += 1;
    }

    pub fn set_current_entity(&mut self, entity: Entity, action_points: i32) {
        self.current_entity = Some(entity);
        self.action_points = action_points;
    }

    pub fn clear_current_entity(&mut self) {
        self.current_entity = None;
        self.action_points = 0;
    }

    pub fn consume_action_points(&mut self, points: i32) -> bool {
        if self.action_points >= points {
            self.action_points -= points;
            true
        } else {
            false
        }
    }
}
