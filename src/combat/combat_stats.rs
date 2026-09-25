use bevy::prelude::*;


#[derive(Component)]
pub struct CombatStats {
    pub attack_speed: f32,
    pub attack_range: f32,
    pub attack_power: f32,
    pub armor: f32,
    pub fire_resist: f32,
    pub frost_resist: f32,
    pub magic_resist: f32,
    pub crit_chance: f32,
    pub crit_multiplier: f32,
}

impl CombatStats {
    pub fn new(
        attack_speed: f32,
        attack_range: f32,
        attack_power: f32,
        armor: f32,
        fire_resist: f32,
        frost_resist: f32,
        magic_resist: f32,
        crit_chance: f32,
        crit_multiplier: f32,
    ) -> Self {
        Self {
            attack_speed,
            attack_range,
            attack_power,
            armor,
            fire_resist,
            frost_resist,
            magic_resist,
            crit_chance,
            crit_multiplier,
        }
    }
}
