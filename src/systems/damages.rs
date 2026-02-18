use crate::systems::health::Health;
use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct DamageEvent {
    pub entity: Entity,
    pub amount: f32,
}

pub fn damage_system(trigger: On<DamageEvent>, mut query: Query<&mut Health>) {
    let entity = trigger.entity.entity();
    let event = trigger.event();

    if let Ok(mut health) = query.get_mut(entity) {
        health.remove_health(event.amount);
    }
}
