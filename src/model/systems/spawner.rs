use bevy::prelude::*;
use bracket_lib::prelude::RandomNumberGenerator;

use crate::model::components::*;
use crate::model::resources::Map;
use crate::RunningState;

pub fn spawn_player(mut commands: Commands, map: Res<Map>) -> Entity {
    // Find a valid position for the player (first floor tile)
    let mut valid_position = Position::new(1, 1);
    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx].is_walkable() {
                valid_position = Position::new(x, y);
                break;
            }
        }
    }

    println!("Spawned player at {:?}", valid_position);

    // Spawn the player
    commands
        .spawn((
            Player,
            valid_position,
            Renderable::new(
                '@',
                Color::srgb(1.0, 1.0, 1.0),
                Color::srgb(0.0, 0.0, 0.0),
                0,
            ),
            FieldOfView::new(8),
            Stats::new(30, 5, 2, 5),
            Name("Player".to_string()),
        ))
        .id()
}

pub fn spawn_monsters(
    mut commands: Commands,
    map: Res<Map>,
    player_query: Query<&Position, With<Player>>,
) {
    let mut rng = RandomNumberGenerator::new();
    let num_monsters = rng.range(5, 15);
    let Ok(player_pos) = player_query.get_single() else {
        return;
    };

    for _ in 0..num_monsters {
        let mut valid_position = None;
        let mut attempts = 0;

        // Find a valid position for the monster
        while valid_position.is_none() && attempts < 100 {
            let x = rng.range(1, map.width - 1);
            let y = rng.range(1, map.height - 1);
            let idx = map.xy_idx(x, y);

            // Check if the position is valid (not blocked and not too close to player)
            if map.tiles[idx].is_walkable() {
                let pos = Position::new(x, y);
                let distance_to_player =
                    ((pos.x - player_pos.x).pow(2) + (pos.y - player_pos.y).pow(2)) as f32;

                if distance_to_player > 25.0 {
                    valid_position = Some(pos);
                }
            }

            attempts += 1;
        }

        if let Some(pos) = valid_position {
            // Randomly choose between two monster types
            if rng.range(0, 100) < 80 {
                // Goblin (more common)
                commands.spawn((
                    Monster,
                    pos,
                    Renderable::new(
                        'g',
                        Color::srgb(0.0, 0.7, 0.0),
                        Color::srgb(0.0, 0.0, 0.0),
                        1,
                    ),
                    FieldOfView::new(6),
                    Stats::new(8, 3, 1, 3),
                    Name("Goblin".to_string()),
                ));
            } else {
                // Orc (less common, stronger)
                commands.spawn((
                    Monster,
                    pos,
                    Renderable::new(
                        'o',
                        Color::srgb(0.7, 0.0, 0.0),
                        Color::srgb(0.0, 0.0, 0.0),
                        1,
                    ),
                    FieldOfView::new(7),
                    Stats::new(15, 4, 2, 2),
                    Name("Orc".to_string()),
                ));
            }
        }
    }
}

pub fn spawn_items(mut commands: Commands, map: Res<Map>) {
    let mut rng = RandomNumberGenerator::new();
    let num_items = rng.range(3, 8);

    for _ in 0..num_items {
        let mut valid_position = None;
        let mut attempts = 0;

        // Find a valid position for the item
        while valid_position.is_none() && attempts < 100 {
            let x = rng.range(1, map.width - 1);
            let y = rng.range(1, map.height - 1);
            let idx = map.xy_idx(x, y);

            if map.tiles[idx].is_walkable() {
                valid_position = Some(Position::new(x, y));
            }

            attempts += 1;
        }

        if let Some(pos) = valid_position {
            // Randomly choose item type
            let item_roll = rng.range(0, 100);

            if item_roll < 70 {
                // Health potion
                commands.spawn((
                    Item,
                    Consumable,
                    pos,
                    Renderable::new(
                        '!',
                        Color::srgb(0.8, 0.0, 0.8),
                        Color::srgb(0.0, 0.0, 0.0),
                        2,
                    ),
                    Name("Health Potion".to_string()),
                ));
            } else if item_roll < 90 {
                // Sword
                commands.spawn((
                    Item,
                    Equippable {
                        slot: EquipmentSlot::MainHand,
                    },
                    pos,
                    Renderable::new(
                        '/',
                        Color::srgb(0.8, 0.8, 0.8),
                        Color::srgb(0.0, 0.0, 0.0),
                        2,
                    ),
                    Name("Sword".to_string()),
                ));
            } else {
                // Shield
                commands.spawn((
                    Item,
                    Equippable {
                        slot: EquipmentSlot::OffHand,
                    },
                    pos,
                    Renderable::new(
                        '[',
                        Color::srgb(0.8, 0.6, 0.2),
                        Color::srgb(0.0, 0.0, 0.0),
                        2,
                    ),
                    Name("Shield".to_string()),
                ));
            }
        }
    }
}

pub fn setup_game(mut map: ResMut<Map>, mut next_state: ResMut<NextState<RunningState>>) {
    println!("Setting up game");

    // Reset the map
    *map = Map::default();

    // Set the game to the Running state
    next_state.set(RunningState::Running);
}

pub fn spawn_initial_player(commands: Commands, map: Res<Map>) {
    println!("Spawning initial player");
    spawn_player(commands, map);
}
