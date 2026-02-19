use crate::{
    combat::attack::AttackEvent,
    player::{ennemy_test::Ennemy, player_setup::Player},
    systems::{death::DeathEvent, health::Health, target::CurrentTarget},
};
use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub amount: f32,
}

pub fn damage_system(
    trigger: On<DamageEvent>,
    mut query: Query<&mut Health>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let entity = event.target;

    if let Ok(mut health) = query.get_mut(entity) {
        health.remove_health(event.amount);
        if health.get_current() <= 0.0 {
            commands.trigger(DeathEvent { target: entity });
        }
    }
}
pub fn test_damages(
    mut command: Commands,
    p_query: Query<Entity, With<Player>>,
    e_query: Query<Entity, With<Ennemy>>,
    keys: Res<ButtonInput<KeyCode>>,
    current_target: Res<CurrentTarget>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        if let Ok(player_entity) = p_query.single() {
            for ennemy_entity in e_query.iter() {
                command.trigger(AttackEvent {
                    attacker: ennemy_entity,
                    target: player_entity,
                });
            }
        }
    }
    if keys.just_pressed(KeyCode::KeyY) {
        if let Ok(player_entity) = p_query.single() {
            if let Some(target_entity) = current_target.entity {
                command.trigger(AttackEvent {
                    attacker: player_entity,
                    target: target_entity,
                });
            }
        }
    }
}
