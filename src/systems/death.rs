use avian3d::dynamics::rigid_body::RigidBody;
use bevy::prelude::*;

use crate::player::player_setup::EntityAnimations;
use crate::systems::animation_names::AnimationNames;
use crate::systems::target::CurrentTarget;

#[derive(Event)]
pub struct DeathEvent {
    pub target: Entity,
}

#[derive(Component)]
pub struct Dead;

pub fn death_system(
    trigger: On<DeathEvent>,
    mut commands: Commands,
    names_assets: Res<Assets<AnimationNames>>,
    animations_query: Query<&EntityAnimations>,
    children_query: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    mut current_target: ResMut<CurrentTarget>,
) {
    let entity = trigger.event().target;

    let Ok(anims) = animations_query.get(entity) else {
        info!("[DEATH] PlayerAnimations not built yet");
        return;
    };

    let Some(names) = names_assets.get(&anims.names) else {
        info!("[DEATH] PlayerAnimations not loaded yet");
        return;
    };

    let Some(death_index) = names.try_get("Death") else {
        info!("[DEATH] PlayerAnimations could not find the 'Death' animation");
        return;
    };
    current_target.entity = None;
    commands.entity(entity).remove::<RigidBody>().insert(Dead);
    for descendant in children_query.iter_descendants(entity) {
        let Ok(mut player) = players.get_mut(descendant) else {
            continue;
        };

        commands
            .entity(descendant)
            .insert(AnimationGraphHandle(anims.graph.clone()));

        player.stop_all();
        player.play(death_index);

        break;
    }
}
