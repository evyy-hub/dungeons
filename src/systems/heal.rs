use bevy::prelude::*;

use crate::{enemies::enemy_setup::Enemy, systems::health::Health};

#[derive(Event)]
pub struct HealEvent {
    pub attacker: Entity,
    pub target: Option<Entity>,
    pub amount: f32,
}

pub fn heal_system(
    trigger: On<HealEvent>,
    mut health_query: Query<&mut Health>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let event = trigger.event();
    // Checking if we heal an ally, or yourself
    let heal_target = match event.target {
        None => event.attacker,
        Some(target) if enemy_query.contains(target) => event.attacker,
        Some(target) => target,
    };

    if let Ok(mut health) = health_query.get_mut(heal_target) {
        if health.get_current() < health.get_max() {
            println!("Healing {:?} amount: {:?}", heal_target, event.amount);
            health.add_health(event.amount);
        }
    }
}
