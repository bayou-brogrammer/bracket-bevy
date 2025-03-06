# Roguelike Game Specifications

This document serves as the main entry point for all specifications related to our Bevy + bracket-lib roguelike game.

## Technical Stack

- Bevy Engine (v0.15.3)
- bracket-lib (latest version with bevy feature)

## Specification Documents

| Category              | Description                                     | Link                                      |
| --------------------- | ----------------------------------------------- | ----------------------------------------- |
| Architecture          | Overall system architecture and design patterns | [Architecture](specs/architecture.md)     |
| Core Game Systems     | Core game mechanics and systems                 | [Core Systems](specs/core_systems.md)     |
| Entity Components     | Entity component system design                  | [Entities](specs/entities.md)             |
| Map Generation        | Dungeon and level generation systems            | [Map Generation](specs/map_generation.md) |
| Combat System         | Combat mechanics and calculations               | [Combat](specs/combat.md)                 |
| UI/UX                 | User interface and experience design            | [UI/UX](specs/ui_ux.md)                   |
| Items & Inventory     | Item system and inventory management            | [Items](specs/items.md)                   |
| AI & Behaviors        | NPC and monster AI systems                      | [AI](specs/ai.md)                         |
| Technical Integration | Bevy and bracket-lib integration details        | [Technical](specs/technical.md)           |
| Performance           | Performance considerations and optimizations    | [Performance](specs/performance.md)       |

## Development Principles

1. Use Bevy as the main game engine and system driver
2. Utilize bracket-lib for roguelike-specific functionality
3. Maintain clean separation of concerns
4. Follow Rust best practices for safety and performance
5. Ensure modular design for future extensibility
