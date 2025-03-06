# Combat System Specification

## Overview

This document details the combat system mechanics and implementation for our roguelike game.

## Core Combat Mechanics

### 1. Stats System

```rust
pub struct CombatStats {
    health: i32,
    max_health: i32,
    attack: i32,
    defense: i32,
    speed: i32,
    critical_chance: f32,
    dodge_chance: f32,
}

pub struct DamageModifier {
    physical: f32,
    fire: f32,
    ice: f32,
    poison: f32,
}
```

### 2. Combat Actions

- Melee Attack
- Ranged Attack
- Special Abilities
- Dodge
- Block
- Use Item

### 3. Damage Types

```rust
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Poison,
    Magic,
}

pub struct Damage {
    amount: i32,
    damage_type: DamageType,
    penetration: f32,
}
```

## Combat Resolution

### 1. Attack Flow

```
Intent → Range Check → Hit Roll → Damage Calculation →
Effects Application → Death Check
```

### 2. Defense Flow

```
Incoming Attack → Dodge Check → Block Check →
Armor Reduction → Damage Application
```

### 3. Status Effects

- Bleeding
- Burning
- Frozen
- Poisoned
- Stunned

## Technical Implementation

### 1. Combat Events

```rust
pub enum CombatEvent {
    AttackIntent {
        attacker: Entity,
        target: Entity,
        weapon: Option<Entity>,
    },
    DamageDealt {
        target: Entity,
        damage: Damage,
        source: Entity,
    },
    Death {
        entity: Entity,
        killer: Option<Entity>,
    },
}
```

### 2. Combat Systems

```rust
pub struct CombatSystem;

impl System for CombatSystem {
    fn run(&mut self, data: SystemData) {
        // Process combat events
        // Update entity states
        // Trigger effects
        // Handle deaths
    }
}
```

## Weapons and Equipment

### 1. Weapon Types

```rust
pub enum WeaponType {
    Sword,
    Axe,
    Spear,
    Bow,
    Staff,
    Dagger,
}

pub struct Weapon {
    weapon_type: WeaponType,
    damage: Damage,
    range: i32,
    speed: i32,
    effects: Vec<StatusEffect>,
}
```

### 2. Armor System

```rust
pub struct Armor {
    physical_defense: i32,
    magical_defense: i32,
    resistances: DamageModifier,
}
```

## Combat Balancing

### 1. Damage Formula

```
final_damage = (base_damage + modifiers) *
               (1 - defense_reduction) *
               resistance_multiplier
```

### 2. Hit Chance Formula

```
hit_chance = base_accuracy +
            attacker_bonuses -
            target_dodge -
            environmental_penalties
```

## Special Abilities

### 1. Ability System

```rust
pub struct Ability {
    name: String,
    cost: i32,
    cooldown: i32,
    effects: Vec<Effect>,
    targeting: TargetingType,
}
```

### 2. Effect System

```rust
pub struct Effect {
    effect_type: EffectType,
    duration: Option<i32>,
    magnitude: f32,
    conditions: Vec<Condition>,
}
```

## Testing Strategy

### 1. Unit Tests

- Damage calculation
- Hit resolution
- Effect application
- Death handling

### 2. Integration Tests

- Combat flow
- Status effects
- Equipment interaction
- Ability usage

### 3. Balance Tests

- Damage output curves
- Time-to-kill metrics
- Survival analysis
- DPS calculations

## Development Guidelines

### 1. Combat Design

- Clear feedback
- Meaningful choices
- Balanced progression
- Interesting interactions

### 2. Performance

- Efficient calculations
- Minimal allocations
- Batched processing
- Event optimization

### 3. Documentation

- Formula explanations
- System interactions
- Balance considerations
- Example scenarios
