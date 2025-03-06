# UI/UX Specification

## Overview

This document details the user interface and user experience design for our roguelike game, focusing on ASCII-based presentation using bracket-lib within the Bevy ecosystem.

## Screen Layout

### 1. Main Game View

```
+------------------------------------------+
|                 Game Map                  |
|                                          |
|                                          |
|                                          |
+------------------+---------------------+
|    Stats Panel   |    Message Log     |
+------------------+---------------------+
```

### 2. UI Components

- Game Map (Main View)
- Stats Panel
- Message Log
- Inventory View
- Character Screen
- Help Screen

## Color Scheme

### 1. Base Colors

```rust
pub const COLORS: Colors = Colors {
    player: RGB::named(YELLOW),
    npc: RGB::named(GREEN),
    monster: RGB::named(RED),
    item: RGB::named(CYAN),
    wall: RGB::named(GRAY),
    floor: RGB::named(DARKGRAY),
    text: RGB::named(WHITE),
}
```

### 2. Status Colors

- Health: Green → Yellow → Red
- Magic: Blue
- Experience: Purple
- Warning: Orange
- Critical: Bright Red

## Input Handling

### 1. Movement

- Arrow Keys / Numpad
- vi keys (h,j,k,l)
- Mouse movement (optional)

### 2. Actions

```rust
pub enum PlayerAction {
    Movement(Direction),
    Attack,
    Pickup,
    UseItem(usize),
    OpenInventory,
    ShowCharacter,
    Help,
    Quit,
}
```

## Message System

### 1. Message Structure

```rust
pub struct Message {
    text: String,
    color: RGB,
    turn: i32,
}

pub struct MessageLog {
    messages: Vec<Message>,
    max_messages: usize,
}
```

### 2. Message Categories

- Combat
- Movement
- Items
- Status Effects
- System Messages

## Menu Systems

### 1. Main Menu

- New Game
- Continue
- Options
- Quit

### 2. Inventory Menu

- Item List
- Item Details
- Usage Options
- Equipment Status

### 3. Character Screen

- Basic Stats
- Equipment
- Status Effects
- Skills/Abilities

## Technical Implementation

### 1. UI Components

```rust
pub struct UIState {
    active_screen: Screen,
    message_log: MessageLog,
    inventory_visible: bool,
    targeting_mode: bool,
}
```

### 2. Rendering System

```rust
pub struct UIRenderSystem;

impl System for UIRenderSystem {
    fn run(&mut self, data: SystemData) {
        // Clear previous frame
        // Draw map
        // Draw entities
        // Draw UI elements
        // Draw active menus
    }
}
```

## Animation System

### 1. Effect Types

- Movement
- Attack
- Spell Cast
- Item Use
- Status Change

### 2. Implementation

```rust
pub struct Animation {
    animation_type: AnimationType,
    frames: Vec<Frame>,
    current_frame: usize,
    speed: f32,
}
```

## Accessibility Features

### 1. Visual

- Configurable colors
- High contrast mode
- Larger ASCII mode

### 2. Input

- Configurable keybindings
- Mouse support
- Simple controls

## Performance Considerations

### 1. Rendering

- Efficient draw calls
- Smart redraw
- Buffer management

### 2. Input

- Input buffering
- Command queueing
- Responsive feedback

## Testing Strategy

### 1. Unit Tests

- Input handling
- Message system
- Menu navigation
- Animation system

### 2. Integration Tests

- UI state management
- Screen transitions
- Input-feedback loop
- Performance metrics

### 3. Usability Tests

- Control scheme
- Information clarity
- Navigation flow
- Feedback timing

## Development Guidelines

### 1. UI Design

- Clear hierarchy
- Consistent style
- Immediate feedback
- Intuitive controls

### 2. UX Principles

- Minimize keystrokes
- Clear feedback
- Forgiving input
- Informative messages

### 3. Documentation

- Control scheme
- UI elements
- Menu structure
- Animation system
