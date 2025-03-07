use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub render_order: i32,
}

impl Renderable {
    pub fn new(glyph: char, fg: Color, bg: Color, render_order: i32) -> Self {
        Self {
            glyph,
            fg,
            bg,
            render_order,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Name(pub String);
