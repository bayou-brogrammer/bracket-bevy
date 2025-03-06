# Map Generation Specification

## Overview

This document outlines the map generation systems and algorithms used in our roguelike game.

## Map Structure

### 1. Tile Types

```rust
pub enum TileType {
    Wall,
    Floor,
    Door,
    Stairs,
    Water,
    Lava,
}

pub struct Tile {
    tile_type: TileType,
    blocked: bool,
    block_sight: bool,
    explored: bool,
}
```

### 2. Map Representation

```rust
pub struct Map {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
    revealed_tiles: Vec<bool>,
    visible_tiles: Vec<bool>,
}
```

## Generation Algorithms

### 1. Room-Based Generation

- Binary Space Partitioning (BSP)
- Random Room Placement
- Cellular Automata Smoothing
- Corridor Connection

### 2. Cave Generation

- Cellular Automata
- Drunkard's Walk
- Noise-based Generation

### 3. Special Areas

- Vaults
- Prefabricated Rooms
- Special Encounters
- Boss Rooms

## Generation Pipeline

### 1. Basic Structure

```
Initial Map → Room Generation → Corridor Creation →
Feature Placement → Decoration → Validation
```

### 2. Room Generation

1. Determine room count
2. Generate room sizes
3. Place rooms
4. Connect rooms
5. Add doors

### 3. Feature Placement

1. Stairs placement
2. Item placement
3. Monster placement
4. Trap placement

## Technical Implementation

### 1. Map Builder

```rust
pub struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    spawn_points: Vec<Point>,
}

impl MapBuilder {
    pub fn new(width: i32, height: i32) -> Self;
    pub fn build(&mut self) -> Map;
    pub fn spawn_points(&self) -> &[Point];
}
```

### 2. Room Generation

```rust
pub struct Room {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self;
    pub fn intersect(&self, other: &Room) -> bool;
    pub fn center(&self) -> (i32, i32);
}
```

## Map Features

### 1. Basic Features

- Rooms
- Corridors
- Doors
- Stairs

### 2. Environmental Features

- Water bodies
- Lava pools
- Chasms
- Bridges

### 3. Special Features

- Secret rooms
- Shops
- Shrines
- Vaults

## Integration with bracket-lib

### 1. Pathfinding

- A\* implementation
- Dijkstra maps
- Flow maps

### 2. Field of View

- Ray casting
- Shadow casting
- Light sources

## Testing Strategy

### 1. Unit Tests

- Room placement
- Corridor connection
- Feature placement
- Pathfinding validation

### 2. Property Tests

- Map connectivity
- Room overlap prevention
- Valid spawn points
- Reachability

### 3. Performance Tests

- Generation speed
- Memory usage
- Pathfinding efficiency

## Development Guidelines

### 1. Map Generation

- Ensure connectivity
- Balance room sizes
- Maintain performance
- Create interesting layouts

### 2. Feature Placement

- Logical distribution
- Balanced difficulty
- Interesting choices
- Clear navigation

### 3. Documentation

- Algorithm explanations
- Usage examples
- Performance considerations
