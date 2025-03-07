# Architecture Specification

## Overview

This document outlines the architecture of our roguelike game built with Bevy and bracket-lib. The architecture follows a modular design with clear separation of concerns, organized around domain-specific plugins that handle different aspects of the game.

## Core Architecture Principles

- **Modularity**: Each domain has its own plugin with clear responsibilities
- **Data-driven**: Uses Bevy ECS for component-based game objects
- **Event-based communication**: Systems communicate through events when possible
- **Clear boundaries**: Minimal dependencies between modules

## Project Structure

```bash
src/
├── main.rs                # Application entry point and core setup
├── app_constants.rs       # Global application constants
├── app_settings.rs        # User-configurable application settings
├── controller/            # Input handling and game commands
│   ├── commands/          # Command pattern implementations
│   ├── components/        # ECS components for input handling
│   ├── events/            # Event definitions for input
│   ├── event_systems/     # Event systems for input handling
│   ├── resources/         # Resources for input state
│   ├── systems/           # Systems for processing input
│   └── controller_plugin.rs # Plugin registration
├── model/                 # Game state and logic
│   ├── commands/          # Game state modification commands
│   ├── components/        # Core game entity components
│   ├── resources/         # Game state resources
│   ├── systems/           # Game logic systems
│   ├── model_constants.rs # Model-specific constants (map dimensions, etc.)
│   └── model_plugin.rs    # Plugin registration
├── view/                  # Rendering and visual representation
│   ├── components/        # Visual components
│   ├── resources/         # Rendering resources
│   ├── systems/           # Rendering systems
│   └── view_plugin.rs     # Plugin registration
├── ui/                    # User interface elements
│   ├── components/        # UI components
│   ├── resources/         # UI state resources
│   ├── systems/           # UI update systems
│   ├── ui_constants.rs    # UI-specific constants (colors, dimensions)
│   └── ui_plugin.rs       # Plugin registration
└── dev/                   # Development tools and debugging features
    └── dev_plugin.rs      # Development plugin with debugging tools
```

## Domain Modules

### Controller Module

**Responsibility**: Handles player input and translates it into game commands.

- Processes keyboard, mouse, and gamepad input
- Manages input state and context
- Dispatches command events to other systems
- Handles input mapping and configuration

### Model Module

**Responsibility**: Manages game state, entities, and core game logic.

- Defines game entities and their components
- Implements game rules and mechanics
- Manages map generation and structure
- Handles game state progression
- Processes AI and NPC behavior
- Maintains game world state

### View Module

**Responsibility**: Renders the game world and entities.

- Manages rendering of game entities
- Handles camera and viewport
- Implements visual effects and animations
- Coordinates with bracket-lib for rendering

### UI Module

**Responsibility**: Manages user interface elements.

- Renders UI components (logs, stats, menus)
- Handles UI layout and positioning
- Manages UI state and interactions
- Displays game information to the player

### Dev Module

**Responsibility**: Provides development and debugging tools.

- Implements debugging visualizations
- Provides development shortcuts
- Enables testing and validation features

## System Execution Order

The game uses Bevy's system sets to ensure proper execution order:

```plaintext
AppSet::RecordInput → AppSet::Visibility → AppSet::Update → AppSet::Render
```

1. **RecordInput**: Capture and process player input
2. **Visibility**: Update visibility and field of view calculations
3. **Update**: Process game logic, AI, and state changes
4. **Render**: Render game world, entities, and UI

## State Management

The game uses a `RunningState` enum to manage high-level game states:

- **Load**: Initial loading state
- **Running**: Normal gameplay
- **Paused**: Game is paused

## Configuration

The game uses a centralized configuration approach:

- **AppSettings**: User-configurable settings (window size, fullscreen, etc.)
- **AppConstants**: Application-wide constants
- **ModelConstants**: Game model specific constants (map dimensions)
- **UiConstants**: UI-specific constants (colors, dimensions)

## Rendering Architecture

The game uses multiple render layers to separate game content from UI:

- **Game Layer (0)**: Game world and entities
- **UI Layer (1)**: User interface elements

## Integration with External Libraries

### Bevy Integration

- Uses Bevy ECS for entity management
- Leverages Bevy's plugin system for modularity
- Utilizes Bevy's event system for communication

### bracket-lib Integration

- Integrated as a Bevy plugin
- Provides roguelike-specific functionality
- Handles field of view and pathfinding

## Development Guidelines

1. **Ownership and Borrowing**: Follow Rust's ownership rules strictly
2. **ECS Patterns**: Use Bevy's ECS patterns consistently
3. **System Boundaries**: Maintain clear boundaries between systems
4. **Documentation**: Document public interfaces thoroughly
5. **Modularity**: Keep related functionality in dedicated modules
6. **Constants**: Use constants for magic numbers and configuration
7. **Error Handling**: Implement proper error handling and recovery
8. **Testing**: Write tests for critical game systems

## Future Considerations

- Save/Load system implementation
- Multiplayer capabilities
- Mod support through plugin architecture
- Performance optimizations for larger maps
- Advanced AI behaviors
