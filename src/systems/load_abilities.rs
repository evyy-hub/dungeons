use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const DEFAULT_SPELLS_TOML: &str = include_str!("../../assets/config/spells.toml");

// Types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DamageType {
    Fire,
    Frost,
    Lightning,
    Physical,
    Holy,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub mana_cost: f32,
    pub cooldown: f32,
    pub range: Option<f32>,
    pub cast_time: f32,
    pub damage: Option<f32>,
    pub damage_type: Option<DamageType>,
    pub healing: Option<f32>,
    pub shield_amount: Option<f32>,
    pub duration: Option<f32>,
    pub area_of_effect: Option<f32>,
}

// Config resource

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct AbilitiesConfig {
    pub abilities: Vec<Ability>,
}

impl Default for AbilitiesConfig {
    fn default() -> Self {
        toml::from_str(DEFAULT_SPELLS_TOML)
            .expect("embedded assets/config/spells.toml is invalid")
    }
}

// Components

#[derive(Component)]
pub struct AbilityCooldown {
    pub ability_name: String,
    pub remaining: f32,
}

// ---------------------------------------------------------------------------
// Debug
// ---------------------------------------------------------------------------
pub fn debug_abilities(config: Res<AbilitiesConfig>) {
    info!("=== Abilities loaded ({}) ===", config.abilities.len());
    for ability in &config.abilities {
        info!(
            "[{}] mana:{} cd:{}s cast:{}s | dmg:{:?} ({:?}) | heal:{:?} | shield:{:?} | range:{:?} | aoe:{:?} | duration:{:?}",
            ability.name,
            ability.mana_cost,
            ability.cooldown,
            ability.cast_time,
            ability.damage,
            ability.damage_type,
            ability.healing,
            ability.shield_amount,
            ability.range,
            ability.area_of_effect,
            ability.duration,
        );
    }
}
