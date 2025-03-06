# Entity Components Specification

## Overview

This document details the entity-component system design for our roguelike game, outlining all major entities and their components.

## Core Components

### 1. Position Components

```rust
pub struct Position {
    x: i32,
    y: i32,
}

pub struct MapLevel(pub i32);
```

### 2. Display Components

```rust
pub struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
    render_order: i32,
}

pub struct Name(pub String);
```

### 3. Actor Components

```rust
pub struct Player;  // Tag component
pub struct Monster;  // Tag component
pub struct NPC;     // Tag component

pub struct Stats {
    health: i32,
    max_health: i32,
    attack: i32,
    defense: i32,
    speed: i32,
}
```

### 4. Item Components

```rust
pub struct Item;  // Tag component
pub struct Consumable;  // Tag component
pub struct Equippable {
    slot: EquipmentSlot,
}

pub struct InBackpack {
    owner: Entity,
}

pub struct Equipped {
    owner: Entity,
    slot: EquipmentSlot,
}
```

## Entity Archetypes

### 1. Player

Required Components:

- Position
- Renderable
- Player
- Stats
- FieldOfView
- Inventory

### 2. Monsters

Required Components:

- Position
- Renderable
- Monster
- Stats
- AI

### 3. Items

Required Components:

- Position
- Renderable
- Item
- Name

### 4. NPCs

Required Components:

- Position
- Renderable
- NPC
- Stats
- AI
- Dialogue

## Component Relationships

### 1. Ownership Chains

```
Player -> Inventory -> Items
Entity -> Equipment -> Items
```

### 2. Spatial Relationships

```
Position -> MapLevel
FieldOfView -> Position
```

## Technical Implementation

### 1. Component Registration

```rust
app.add_plugin(ComponentsPlugin)
   .register_type::<Position>()
   .register_type::<Stats>()
   // ... other components
```

### 2. Systems Integration

- Component change detection
- Component serialization
- Component cleanup

### 3. Performance Considerations

- Component storage strategy
- Query optimization
- Archetype organization

## Testing Strategy

### 1. Unit Tests

- Component initialization
- Component relationships
- Component systems

### 2. Integration Tests

- Entity creation
- Component interaction
- System integration

### 3. Performance Tests

- Component access patterns
- Memory usage
- Query performance

## Development Guidelines

### 1. Component Design

- Keep components focused and minimal
- Use appropriate data types
- Consider cache efficiency

### 2. System Integration

- Clear system dependencies
- Efficient queries
- Proper change detection

### 3. Documentation

- Clear component purpose
- Usage examples
- System requirements
