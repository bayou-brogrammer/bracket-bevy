use bevy::prelude::*;
use std::collections::VecDeque;

use crate::ui::UiConstants;

const DEFAULT_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);

#[derive(Debug, Clone, Reflect)]
pub struct LogEntry {
    pub text: String,
    pub color: Color,
}

impl LogEntry {
    pub fn new(text: impl Into<String>, color: Color) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }
}

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameLog {
    pub entries: VecDeque<LogEntry>,
}

impl Default for GameLog {
    fn default() -> Self {
        let mut log = Self {
            entries: VecDeque::with_capacity(UiConstants::LOG_LIMIT),
        };
        log.add_entry_srgb("Welcome to the dungeon!", DEFAULT_COLOR);
        log
    }
}

impl GameLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, text: impl Into<String>) {
        self.entries.push_back(LogEntry::new(text, DEFAULT_COLOR));
        if self.entries.len() > UiConstants::LOG_LIMIT {
            self.entries.pop_front();
        }
    }

    pub fn add_entry_srgb(&mut self, text: impl Into<String>, color: Color) {
        self.entries.push_back(LogEntry::new(text, color));
        if self.entries.len() > UiConstants::LOG_LIMIT {
            self.entries.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
