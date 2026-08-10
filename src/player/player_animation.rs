use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::{
    player::player_setup::{EntityAnimations, Player, PlayerState},
    systems::{animation_names::AnimationNames, death::Dead},
};

#[derive(Component)]
pub struct CurrentAnim {
    pub index: Option<AnimationNodeIndex>,
}
// Probably deprecated, but leaving it for now in case we need it later.
pub fn setup_player_anim(
    mut commands: Commands,
    anim_players: Query<(Entity, &ChildOf), Added<AnimationPlayer>>,
    players: Query<(Entity, &EntityAnimations), With<Player>>,
    parents: Query<&ChildOf>,
) {
    for (anim_entity, child_of) in &anim_players {
        let mut current = child_of.get();

        loop {
            if let Ok(anims) = players.get(current) {
                // AnimationGraphHandle must be on the entity that actually
                // holds AnimationPlayer, not on the root entity.
                commands
                    .entity(anim_entity)
                    .insert(AnimationGraphHandle(anims.1.graph.clone()));

                break;
            }

            let Ok(parent) = parents.get(current) else {
                break;
            };

            current = parent.get();
        }
    }
}
// Update the player's animation based on their current state (Idle or Running).
pub fn update_player_animation(
    mut q: Query<(Entity, &Player, &EntityAnimations), Without<Dead>>,
    children_query: Query<&Children>,
    mut anim_players: Query<&mut AnimationPlayer>,
    names_assets: Res<Assets<AnimationNames>>,
) {
    for (player_entity, player, anims) in &mut q {
        let Some(names) = names_assets.get(&anims.names) else {
            continue;
        };

        let target_name = match player.state {
            PlayerState::Idle => "Idle",
            PlayerState::Running => "Run",
        };

        let Some(target) = names.try_get(target_name) else {
            continue;
        };

        for descendant in children_query.iter_descendants(player_entity) {
            let Ok(mut anim_player) = anim_players.get_mut(descendant) else {
                continue;
            };

            if anim_player.is_playing_animation(target) {
                break;
            }

            anim_player.stop_all();
            anim_player.play(target).repeat();

            break;
        }
    }
}
