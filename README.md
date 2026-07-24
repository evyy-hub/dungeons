# Dungeons

A dungeon crawler built with Bevy and Rust.

## Features

- Combat system built around observer events: `CastEvent → AttackEvent → DamageEvent → DeathEvent`
- Spell/ability system driven by TOML config via a `SpellBook` component
- HP/mana UI (no main menu/state machine yet)
- Physics and hitboxes powered by Avian3D, with mesh outlining (`bevy_mod_outline`) and mesh picking

## Status

Personal, ongoing project — actively developed, no fixed release plan. See [TODO.md](./TODO.MD) for what's next.
