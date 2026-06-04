use bevy::prelude::*;
use rand::Rng;

use super::combat_stats::CombatStats;
use crate::systems::damages::DamageEvent;

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
}

pub fn attack_system(trigger: On<AttackEvent>, query: Query<&CombatStats>, mut commands: Commands) {
    let event = trigger.event();
    let attacker_entity = event.attacker;
    let target_entity = event.target;

    println!(
        "[ATTACK_SYSTEM] Received AttackEvent: {:?} -> {:?}",
        attacker_entity, target_entity
    );

    let Ok(attacker_stats) = query.get(attacker_entity) else {
        println!("[ATTACK_SYSTEM] ERROR: Attacker has no CombatStats!");
        return;
    };
    let Ok(target_stats) = query.get(target_entity) else {
        println!("[ATTACK_SYSTEM] ERROR: Target has no CombatStats!");
        return;
    };

    let mut damage = attacker_stats.attack_power;

    let mut rng = rand::rng();
    let is_crit = rng.random::<f32>() < attacker_stats.crit_chance;
    if is_crit {
        damage *= attacker_stats.crit_multiplier;
    }

    // Apply armor damage reduction formula: reduction = armor / (armor + 100)
    let damage_reduction = target_stats.armor / (target_stats.armor + 100.0);
    let final_damage = damage * (1.0 - damage_reduction);

    println!(
        "[ATTACK_SYSTEM] Damage calculation: base={}, crit={}, armor_reduction={:.1}%, final={:.1}",
        attacker_stats.attack_power,
        is_crit,
        damage_reduction * 100.0,
        final_damage
    );

    println!("[ATTACK_SYSTEM] Triggering DamageEvent");
    commands.trigger(DamageEvent {
        attacker: attacker_entity,
        target: target_entity,
        amount: final_damage,
    });
}
