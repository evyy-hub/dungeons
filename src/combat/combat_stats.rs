use bevy::prelude::*;

#[derive(Component)]
pub struct CombatStats {
    pub attack_power: f32,
    pub armor: f32,
    pub crit_chance: f32,
    pub crit_multiplier: f32,
}

impl CombatStats {
    pub fn new(attack_power: f32, armor: f32, crit_chance: f32, crit_multiplier: f32) -> Self {
        Self {
            attack_power,
            armor,
            crit_chance,
            crit_multiplier,
        }
    }
}
