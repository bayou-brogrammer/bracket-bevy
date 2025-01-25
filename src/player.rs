use crate::components::*;
use crate::map::*;
use crate::AppSet;
use bevy::prelude::*;
use bracket_lib::bevy::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player)
        .add_systems(Update, move_player.in_set(AppSet::RecordInput));
}

fn spawn_player(mut commands: Commands, map: Res<Map>) {
    let (x, y) = map.rooms[0].center();
    commands
        .spawn_empty()
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Position { x, y })
        .insert(Player);
}

fn move_player(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player_pos: Query<&mut Position, With<Player>>,
    map: Res<Map>,
) {
    if let Some(kbi) = kb_input.get_pressed().next() {
        let (player_x, player_y) = match kbi {
            KeyCode::ArrowLeft | KeyCode::KeyA | KeyCode::KeyH | KeyCode::Numpad4 => (-1, 0),
            KeyCode::ArrowRight | KeyCode::KeyD | KeyCode::KeyL | KeyCode::Numpad6 => (1, 0),
            KeyCode::ArrowUp | KeyCode::KeyW | KeyCode::KeyK | KeyCode::Numpad8 => (0, -1),
            KeyCode::ArrowDown | KeyCode::KeyS | KeyCode::KeyJ | KeyCode::Numpad2 => (0, 1),
            _ => (0, 0),
        };

        if let Some(mut pos) = player_pos.single_mut().into() {
            let destination_idx = xy_idx(pos.x + player_x, pos.y + player_y);

            if map.tiles[destination_idx] != TileType::Wall {
                pos.x = (pos.x + player_x).clamp(0, 79);
                pos.y = (pos.y + player_y).clamp(0, 49);
            }
        }
    }
}
