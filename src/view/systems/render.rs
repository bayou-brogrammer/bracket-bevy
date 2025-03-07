use bevy::prelude::*;
use bracket_lib::bevy::*;
use bracket_lib::prelude::{FontCharType, RGB};

use crate::model::components::{Position, Renderable, TerrainType};
use crate::model::resources::Map;
use crate::RunningState;

pub fn render_map(map: &Res<Map>, ctx: &Res<BracketContext>) {
    let mut draw_batch = ctx.new_draw_batch();

    // Clear the console
    draw_batch.cls();

    // Render the map
    for y in 0..map.height {
        for x in 0..map.width {
            let idx = map.xy_idx(x, y);
            let tile = &map.tiles[idx];

            // Only render if the tile has been revealed
            if map.revealed_tiles[idx] {
                let glyph: FontCharType;
                let fg: RGB;
                let bg = RGB::from_f32(0.0, 0.0, 0.0);

                match tile {
                    TerrainType::Wall => {
                        glyph = to_cp437('#');
                        fg = RGB::from_f32(0.5, 0.5, 0.5);
                    }
                    TerrainType::Floor => {
                        glyph = to_cp437('.');
                        fg = RGB::from_f32(0.3, 0.3, 0.3);
                    }
                    TerrainType::Stairs => {
                        glyph = to_cp437('>');
                        fg = RGB::from_f32(1.0, 1.0, 1.0);
                    }
                }

                // If the tile is currently visible, use brighter colors
                if map.visible_tiles[idx] {
                    draw_batch.set(Point::new(x, y), ColorPair::new(fg * 1.5, bg), glyph);
                } else {
                    // Dimmer colors for explored but not visible tiles
                    draw_batch.set(Point::new(x, y), ColorPair::new(fg * 0.5, bg), glyph);
                }
            }
        }
    }

    ctx.submit_batch(0, draw_batch);
}

pub fn render_entities(
    map: &Res<Map>,
    ctx: &Res<BracketContext>,
    query: &Query<(&Position, &Renderable)>,
) {
    let mut draw_batch = ctx.new_draw_batch();

    // Sort entities by render order
    let mut entities: Vec<(&Position, &Renderable)> = query.iter().collect();
    entities.sort_by(|a, b| a.1.render_order.cmp(&b.1.render_order));

    // Render entities
    for (pos, render) in entities {
        let idx = map.xy_idx(pos.x, pos.y);

        // Only render if the entity is in a visible tile
        if map.visible_tiles[idx] {
            draw_batch.set(
                Point::new(pos.x, pos.y),
                ColorPair::new(render.fg, render.bg),
                to_cp437(render.glyph),
            );
        }
    }

    ctx.submit_batch(5000, draw_batch);
}

pub fn render_ui(map: Res<Map>, ctx: Res<BracketContext>) {
    let mut draw_batch = ctx.new_draw_batch();

    // Clear the console
    // draw_batch.cls();

    // Draw a box around the map
    draw_batch.draw_hollow_box(
        bracket_lib::prelude::Rect::with_size(0, 0, map.width - 1, map.height - 1),
        ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0)),
    );

    // Add UI elements here

    ctx.submit_batch(10000, draw_batch);
}

pub fn render_system(
    map: Res<Map>,
    ctx: Res<BracketContext>,
    query: Query<(&Position, &Renderable)>,
    state: Res<State<RunningState>>,
) {
    if *state.get() != RunningState::Running {
        return;
    }

    // Clear the screen
    ctx.cls();

    // Render map, entities, and UI
    render_map(&map, &ctx);
    render_entities(&map, &ctx, &query);
    render_ui(map, ctx);

    // Present the frame
    // ctx.present().expect("Failed to present frame");
}
