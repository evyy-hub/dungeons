use crate::{
    combat::attack::AttackEvent,
    ennemies::enemy_test::Enemy,
    player::player_setup::Player,
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

    println!(
        "[DAMAGE_SYSTEM] Received DamageEvent: {} to {:?}",
        event.amount, entity
    );

    if let Ok(mut health) = query.get_mut(entity) {
        println!(
            "[DAMAGE_SYSTEM] Applying damage: {} to entity {:?}",
            event.amount, entity
        );
        health.remove_health(event.amount);
        if health.get_current() <= 0.0 {
            println!(
                "[DAMAGE_SYSTEM] Entity {:?} is dead, triggering DeathEvent",
                entity
            );
            commands.trigger(DeathEvent { target: entity });
        }
    } else {
        println!(
            "[DAMAGE_SYSTEM] ERROR: Entity {:?} has no Health component!",
            entity
        );
    }
}
pub fn test_damages(
    mut command: Commands,
    p_query: Query<Entity, With<Player>>,
    e_query: Query<Entity, With<Enemy>>,
    keys: Res<ButtonInput<KeyCode>>,
    current_target: Res<CurrentTarget>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        println!("[T] Triggering auto-attack for all enemies");
        if let Ok(player_entity) = p_query.single() {
            for enemy_entity in e_query.iter() {
                command.trigger(AttackEvent {
                    attacker: enemy_entity,
                    target: player_entity,
                    spell_damage: 15.0,
                });
            }
        }
    }
    if keys.just_pressed(KeyCode::KeyY) {
        println!(
            "[Y] Attempting player attack, current_target: {:?}",
            current_target.entity
        );
        if let Ok(player_entity) = p_query.single() {
            if let Some(target_entity) = current_target.entity {
                println!(
                    "[Y] Triggering attack: player {:?} -> target {:?}",
                    player_entity, target_entity
                );
                command.trigger(AttackEvent {
                    attacker: player_entity,
                    target: target_entity,
                    spell_damage: 15.0,
                });
            } else {
                println!("[Y] No target selected!");
            }
        }
    }
}
