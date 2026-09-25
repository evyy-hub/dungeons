use avian3d::prelude::*;
use bevy::prelude::*;

use crate::combat::attack::AttackEvent;
use crate::systems::death::Dead;
use crate::systems::keys::{Action, Actions};
use crate::{
    combat::combat_stats::CombatStats, player::player_setup::Player, systems::target::CurrentTarget,
};
#[derive(Component)]
pub struct AutoAttack {
    pub timer: Timer,
    pub active: bool,
}
pub fn auto_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    spatial: SpatialQuery,
    current_target: Res<CurrentTarget>,
    mut query: Query<
        (Entity, &Transform, &CombatStats, &mut AutoAttack),
        (With<Player>, Without<Dead>),
    >,
    targets: Query<&Transform, Without<Dead>>,
    parents: Query<&ChildOf>,
) {
    for (entity, tf, stats, mut auto) in &mut query {
        auto.timer.tick(time.delta());

        if !auto.active {
            continue;
        }

        let Some(target) = current_target.entity else {
            info!("[AUTO] no target, disabling");
            auto.active = false;
            continue;
        };
        let Ok(target_tf) = targets.get(target) else {
            info!("[AUTO] target not found or dead (target = {:?})", target);
            auto.active = false;
            continue;
        };
        if !auto.timer.is_finished() {
            continue;
        }
        let origin = tf.translation + Vec3::Y * 1.0;
        let dest = target_tf.translation + Vec3::Y * 1.0;
        info!(
            "[AUTO] dist = {:.2}, range = {}",
            origin.distance(dest),
            stats.attack_range
        );

        if !can_hit(
            &spatial,
            &parents,
            entity,
            tf,
            target,
            target_tf,
            stats.attack_range,
        ) {
            info!("[AUTO] can_hit = false");
            continue;
        }

        auto.timer = Timer::from_seconds(stats.attack_speed, TimerMode::Once);

        commands.trigger(AttackEvent {
            attacker: entity,
            target,
            spell_damage: stats.attack_power,
        });
    }
}

pub fn can_hit(
    spatial: &SpatialQuery,
    parents: &Query<&ChildOf>,
    attacker: Entity,
    attacker_tf: &Transform,
    target: Entity,
    target_tf: &Transform,
    range: f32,
) -> bool {
    let origin = attacker_tf.translation + Vec3::Y * 1.0;
    let dest = target_tf.translation + Vec3::Y * 1.0;

    let to_target = dest - origin;
    if to_target.length() > range + 2.0 {
        return false;
    }
    let Ok(dir) = Dir3::new(to_target) else {
        return true;
    };

    let filter = SpatialQueryFilter::from_excluded_entities([attacker]);

    match spatial.cast_ray(origin, dir, range, true, &filter) {
        Some(hit) => {
            hit.entity == target || parents.iter_ancestors(hit.entity).any(|a| a == target)
        }
        None => false,
    }
}
pub fn toggle_auto_attack(
    mut query: Query<&mut AutoAttack, (With<Player>, Without<Dead>)>,
    current_target: Res<CurrentTarget>,
    action: Res<Actions>,
) {
    if !action.just_pressed(Action::ToggleAutoAttack) {
        return;
    }
    let Ok(mut auto) = query.single_mut() else {
        return;
    };

    // No target: cannot activate
    if !auto.active && current_target.entity.is_none() {
        info!("[AUTO_ATTACK] No target");
        return;
    }

    auto.active = !auto.active;
    info!("[AUTO_ATTACK] {}", if auto.active { "ON" } else { "OFF" });
}
