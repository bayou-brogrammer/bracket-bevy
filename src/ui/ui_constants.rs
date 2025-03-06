use bevy::{color::palettes::css, prelude::*, render::view::RenderLayers};

pub struct UiConstants;

impl UiConstants {
    /// UI layout controlling the height of the `GameLog`
    pub const LOG_HEIGHT: u32 = 100;
    /// UI layout controlling the width of the stats panel
    pub const STATS_WIDTH: u32 = 300;

    /// The maximum number of messages to display in the `GameLog`
    pub const LOG_LIMIT: usize = 50;
    /// The background color of the `GameLog`
    pub const LOG_BACKGROUND: Srgba = Srgba::new(0.2, 0.2, 0.2, 1.0);
    pub const LOG_FOREGROUND: Srgba = Srgba::new(0.0, 1.0, 0.0, 1.0);

    /// The background color of the stats panel
    pub const STATS_BACKGROUND: Srgba = css::SLATE_BLUE;

    /// This is the default layer, entities are added to this layer implicitly
    pub const GAME_LAYER: RenderLayers = RenderLayers::layer(0);
    /// This is the UI layer, entities shouldn't be assigned to this layer use `TargetCamera` for root UI nodes
    pub const UI_LAYER: RenderLayers = RenderLayers::layer(1);
}
