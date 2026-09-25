use avian3d::parry::utils::hashmap::HashMap;
use bevy::prelude::*;

use crate::{
    combat::attack::AttackEvent,
    player::player_setup::{Player, SpellBook},
    systems::{
        death::Dead,
        heal::HealEvent,
        keys::{Action, Actions},
        load_abilities::AbilitiesConfig,
        mana::Mana,
        target::CurrentTarget,
    },
};

#[derive(Event)]
pub struct CastEvent {
    pub caster: Entity,
    pub target: Option<Entity>,
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
        warn!("[CAST] Couldn't find  '{}' ", event.ability_name);
        return;
    };
    if ability.damage.is_some() && event.target.is_none() {
        info!("[CAST] {} need a target", ability.name);
        return;
    }

    let Ok(mut cooldowns) = cooldowns_query.get_mut(event.caster) else {
        warn!("[CAST] No cooldowns");
        return;
    };

    if let Some(timer) = cooldowns.timers.get(&event.ability_name) {
        if timer.remaining_secs() > 0.0 {
            info!("[CAST] cooldown :{}", timer.remaining_secs());
            return;
        }
    }

    let Ok(mut mana) = caster_query.get_mut(event.caster) else {
        warn!("[CAST] No mana");
        return;
    };

    if mana.get_current() < ability.mana_cost {
        info!("[CAST] Not enough mana  :{}", mana.get_current());
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
        "[CAST] {} cast {} on {:?}",
        event.caster, ability.name, event.target
    );
    if let Some(_damage) = ability.damage {
        let Some(target) = event.target else {
            warn!("[CAST] NO TARGET");
            return;
        };

        if let Some(damage) = ability.damage {
            commands.trigger(AttackEvent {
                attacker: event.caster,
                target,
                spell_damage: damage,
            });
        }
    }
    if let Some(heal) = ability.healing {
        commands.trigger(HealEvent {
            attacker: event.caster,
            target: event.target,
            amount: heal,
        });
    }
}
pub fn cast_from_input(
    mut commands: Commands,
    p_query: Query<(&SpellBook, Entity), (With<Player>, Without<Dead>)>,
    current_target: Res<CurrentTarget>,
    actions: Res<Actions>,
) {
    let Ok((spellbook, player_entity)) = p_query.single() else {
        return;
    };
    let target_entity = current_target.entity;

    const SLOTS: [Action; 4] = [
        Action::Spell1,
        Action::Spell2,
        Action::Spell3,
        Action::Spell4,
    ];

    let Some(slot) = SLOTS.iter().position(|a| actions.just_pressed(*a)) else {
        return;
    };

    match spellbook.spells.get(slot) {
        Some(name) => {
            info!("[SPELLBOOK] Spell: {:?}", name);
            commands.trigger(CastEvent {
                caster: player_entity,
                target: current_target.entity,
                ability_name: name.clone(),
            });
        }
        None => info!("[CAST] Slot {} empty", slot + 1),
    }
}

pub fn tick_cooldowns(mut query: Query<&mut SpellCooldowns>, time: Res<Time>) {
    for mut cooldowns in &mut query {
        for timer in cooldowns.timers.values_mut() {
            timer.tick(time.delta());
        }
    }
}
