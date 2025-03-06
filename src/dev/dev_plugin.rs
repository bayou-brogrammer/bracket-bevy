use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin};

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Inspector
        app.add_plugins((DefaultInspectorConfigPlugin, WorldInspectorPlugin::new()));

        // Frame Diagnostics
        app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
    }
}
