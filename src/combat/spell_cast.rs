use avian3d::parry::utils::hashmap::HashMap;
use bevy::prelude::*;

use crate::{
    combat::attack::AttackEvent,
    player::player_setup::{Player, SpellBook},
    systems::{load_abilities::AbilitiesConfig, mana::Mana, target::CurrentTarget},
};

#[derive(Event)]
pub struct CastEvent {
    pub caster: Entity,
    pub target: Entity,
    pub ability_name: String,
}

#[derive(Component, Default)]
pub struct SpellCooldowns {
    pub timers: HashMap<String, Timer>,
}

pub fn on_cast(
    trigger: On<CastEvent>,
    mut commands: Commands,
    config: Res<AbilitiesConfig>,
    mut cooldowns_query: Query<&mut SpellCooldowns>,
    mut caster_query: Query<&mut Mana>,
) {
    let event = trigger.event();

    let Some(ability) = config
        .abilities
        .iter()
        .find(|a| a.name == event.ability_name)
    else {
        warn!("[CAST] Sort '{}' introuvable", event.ability_name);
        return;
    };

    let Ok(mut cooldowns) = cooldowns_query.get_mut(event.caster) else {
        warn!("[CAST] Pas de cooldowns");
        return;
    };

    if let Some(timer) = cooldowns.timers.get(&event.ability_name) {
        if timer.remaining_secs() > 0.0 {
            info!("[CAST] cooldown actif :{}", timer.remaining_secs());
            return;
        }
    }

    let Ok(mut mana) = caster_query.get_mut(event.caster) else {
        warn!("[CAST] Pas de mana");
        return;
    };

    if mana.get_current() < ability.mana_cost {
        info!("[CAST] mana insuffisant:{}", mana.get_current());
        return;
    }

    mana.remove_mana(ability.mana_cost);

    if ability.cooldown > 0.0 {
        cooldowns.timers.insert(
            event.ability_name.clone(),
            Timer::from_seconds(ability.cooldown, TimerMode::Once),
        );
    }

    info!(
        "[CAST] {} cast {} sur {:?}",
        event.caster, ability.name, event.target
    );

    if let Some(damage) = ability.damage {
        commands.trigger(AttackEvent {
            attacker: event.caster,
            target: event.target,
            spell_damage: damage,
        });
    }
}

pub fn cast_from_input(
    mut commands: Commands,
    p_query: Query<(&SpellBook, Entity), With<Player>>,
    current_target: Res<CurrentTarget>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((spellbook, player_entity)) = p_query.single() else {
        return;
    };
    let Some(target_entity) = current_target.entity else {
        return;
    };

    // Touche 1-4 → cast le sort à l'index correspondant
    let slot: Option<usize> = if keys.just_pressed(KeyCode::KeyJ) {
        Some(0)
    } else if keys.just_pressed(KeyCode::KeyK) {
        Some(1)
    } else if keys.just_pressed(KeyCode::KeyL) {
        Some(2)
    } else if keys.just_pressed(KeyCode::KeyM) {
        Some(3)
    } else {
        None
    };

    if let Some(i) = slot {
        if let Some(name) = spellbook.spells.get(i) {
            info!("[SPELLBOOK] Spells: {:?}", name);
            commands.trigger(CastEvent {
                caster: player_entity,
                target: target_entity,
                ability_name: name.clone(),
            });
        } else {
            info!("[CAST] Slot {} vide", i + 1);
        }
    }
}

pub fn tick_cooldowns(mut query: Query<&mut SpellCooldowns>, time: Res<Time>) {
    for mut cooldowns in &mut query {
        for timer in cooldowns.timers.values_mut() {
            timer.tick(time.delta());
        }
    }
}
