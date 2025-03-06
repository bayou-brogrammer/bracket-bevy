# Technical Integration Specification

## Overview

This document details the technical integration between Bevy 0.15.3 and bracket-lib, outlining how these two libraries work together in our roguelike game.

## Core Integration Points

### 1. Plugin Structure

```rust
pub struct BracketLibPlugin;

impl Plugin for BracketLibPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BTermBuilder::simple()?)
           .add_system(bracket_render_system)
           .add_system(input_system)
           .init_resource::<BracketContext>();
    }
}
```

### 2. Rendering Pipeline

```rust
pub struct BracketContext {
    context: BTerm,
    console: Console,
}

pub fn bracket_render_system(
    mut context: ResMut<BracketContext>,
    query: Query<(&Position, &Renderable)>,
) {
    // Clear console
    // Draw map
    // Draw entities
    // Present frame
}
```

## Component Integration

### 1. Position Mapping

```rust
// Bevy Component
pub struct Position {
    x: i32,
    y: i32,
}

// bracket-lib Point conversion
impl From<Position> for Point {
    fn from(pos: Position) -> Self {
        Point::new(pos.x, pos.y)
    }
}
```

### 2. Color Integration

```rust
// Color mapping between Bevy and bracket-lib
pub fn convert_color(bevy_color: Color) -> RGB {
    RGB::from_f32(
        bevy_color.r(),
        bevy_color.g(),
        bevy_color.b(),
    )
}
```

## Systems Integration

### 1. Input Handling

```rust
pub fn input_system(
    context: Res<BracketContext>,
    mut commands: Commands,
) {
    if let Some(key) = context.context.key {
        match key {
            // Handle input
        }
    }
}
```

### 2. Map Generation

```rust
pub struct MapGeneration {
    pub builder: MapBuilder,
    pub algorithm: Box<dyn MapAlgorithm>,
}

impl MapGeneration {
    pub fn build(&mut self) -> Map {
        // Use bracket-lib's map generation
        // Convert to our map format
    }
}
```

## Performance Optimization

### 1. Render Batching

```rust
pub struct RenderBatch {
    glyphs: Vec<(Point, FontCharType, RGB, RGB)>,
    dirty: bool,
}

impl RenderBatch {
    pub fn flush(&mut self, ctx: &mut BTerm) {
        // Batch render operations
    }
}
```

### 2. Cache Management

```rust
pub struct RenderCache {
    map_cache: HashMap<Point, (char, RGB, RGB)>,
    entity_cache: HashMap<Entity, (char, RGB, RGB)>,
}
```

## Resource Management

### 1. Console Management

```rust
pub struct ConsoleManager {
    main_console: Console,
    ui_console: Console,
    popup_console: Option<Console>,
}
```

### 2. Asset Integration

```rust
pub struct BracketAssets {
    tilesets: HashMap<String, Tileset>,
    fonts: HashMap<String, Font>,
}
```

## Error Handling

### 1. Error Types

```rust
pub enum BracketError {
    InitializationError(String),
    RenderError(String),
    ResourceError(String),
}
```

### 2. Result Types

```rust
pub type BracketResult<T> = Result<T, BracketError>;
```

## Testing Infrastructure

### 1. Mock Components

```rust
pub struct MockBTerm {
    key_queue: VecDeque<VirtualKeyCode>,
    drawn_chars: Vec<(Point, char, RGB, RGB)>,
}
```

### 2. Test Utilities

```rust
pub struct TestContext {
    app: App,
    bracket_context: BracketContext,
}
```

## Development Guidelines

### 1. Bevy Integration

- Use Bevy's ECS as primary architecture
- Leverage Bevy's plugin system
- Follow Bevy's resource patterns
- Use Bevy's event system

### 2. bracket-lib Usage

- Utilize for roguelike specifics
- Map generation
- FOV calculations
- Path finding

### 3. Performance

- Minimize context switches
- Batch render operations
- Cache when appropriate
- Profile critical paths

### 4. Error Handling

- Graceful degradation
- Clear error messages
- Recovery strategies
- Logging integration

## Version Compatibility

### 1. Bevy Requirements

- Version: 0.15.3
- Required features
- Plugin compatibility

### 2. bracket-lib Requirements

- Latest version
- Bevy feature enabled
- Required dependencies

## Documentation Requirements

### 1. Code Documentation

- Public API documentation
- Integration points
- System relationships
- Error conditions

### 2. Examples

- Basic integration
- Common patterns
- Error handling
- Performance optimization
