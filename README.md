# Dungeons

A 3D dungeon crawler prototype built with Bevy and Rust.

## Features

* Event-driven combat pipeline: `CastEvent → AttackEvent → DamageEvent → DeathEvent`
* Data-driven spell system powered by TOML and a `SpellBook` component
* Health and mana system
* Third-person camera with rotation and zoom
* Target selection using mesh picking
* Physics and hitboxes powered by Avian3D
* Mesh outlining with `bevy_mod_outline`
* Combat damage calculations with critical strikes and armor mitigation
* Generic enemy spawning system
* Data-driven animation setup using generated RON animation graphs and animation name mappings
* Universal death animation system shared between players and enemies

## Assets

Character and enemy 3D models are provided by [Quaternius](https://quaternius.com/).

The project uses Quaternius assets as a base for prototyping gameplay and game systems.

## Combat System

The combat system is built around an event-driven architecture:

```text
CastEvent → AttackEvent → DamageEvent → DeathEvent
```

Damage resolution happens during the `AttackEvent` phase inside `combat::attack`.

The damage calculation takes into account:

* Base spell damage
* Critical hit chance
* Critical damage multiplier
* Target armor mitigation

Damage calculation flow:

```text
Base Damage
    ↓
Critical Hit Check
    ↓
Critical Multiplier (if triggered)
    ↓
Armor Damage Reduction
    ↓
Final Damage
```

Critical hits use the attacker's `crit_chance`.

When a critical hit occurs, the damage is multiplied by the attacker's `crit_multiplier`.

Armor reduces incoming damage using:

```text
damage_reduction = armor / (armor + 100)
```

Final damage is calculated with:

```text
final_damage = damage * (1 - damage_reduction)
```

Example:

```text
Base damage: 100
Critical multiplier: x2
Target armor: 50

Critical damage:
100 × 2 = 200

Armor reduction:
50 / (50 + 100) = 33.3%

Final damage:
200 × (1 - 0.333) = 133.3 damage
```

## Controls

| Key                          | Action                                                |
| ---------------------------- | ----------------------------------------------------- |
| `Z Q S D`                    | Move                                                  |
| `Right Mouse Button` + Mouse | Rotate camera                                         |
| `Mouse Wheel`                | Zoom camera                                           |
| `Left Mouse Button`          | Select target                                         |
| `Y`                          | Basic attack on nearby enemies                        |
| `T`                          | Trigger nearby enemies to attack the player *(debug)* |
| `J`                          | Cast spell (Fireball)                                 |
| `K`                          | Cast spell (Heal - placeholder)                       |
| `L`                          | Spell slot 3 *(empty)*                                |
| `M`                          | Spell slot 4 *(empty)*                                |

> **Note:** The Heal spell is available but its gameplay effect is not implemented yet. Spell slots 3 and 4 are currently placeholders.

## Getting Started

### Requirements

* Rust
* Cargo

### Run

```bash
cargo run
```

## Status

Personal, ongoing project built to explore game architecture with Bevy ECS.

Current focus:

* Expanding the combat system
* Enemy AI
* Melee combat
* Character classes
* Experience system

See [TODO.md](./TODO.md) for the full roadmap.
