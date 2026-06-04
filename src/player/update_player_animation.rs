use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::player::player_setup::{Player, PlayerAnimations, PlayerState};
#[derive(Component)]
pub struct PlayerAnim {
    pub anim_player: Entity,
}
#[derive(Component)]
pub struct CurrentAnim {
    pub index: Option<AnimationNodeIndex>,
}
pub fn setup_player_anim(
    mut commands: Commands,
    anim_players: Query<(Entity, &ChildOf), Added<AnimationPlayer>>,
    players: Query<Entity, With<Player>>,
    parents: Query<&ChildOf>,
) {
    for (anim_entity, child_of) in &anim_players {
        let mut current = child_of.get();

        loop {
            if players.get(current).is_ok() {
                commands.entity(current).insert(PlayerAnim {
                    anim_player: anim_entity,
                });
                break;
            }

            let Ok(parent) = parents.get(current) else {
                break;
            };

            current = parent.get();
        }
    }
}
pub fn update_player_animation(
    mut q: Query<(&Player, &PlayerAnimations, &PlayerAnim, &mut CurrentAnim)>,
    mut anim_players: Query<&mut AnimationPlayer>,
) {
    for (player, anims, anim, mut current) in &mut q {
        let Ok(mut anim_player) = anim_players.get_mut(anim.anim_player) else {
            continue;
        };

        let target = match player.state {
            PlayerState::Idle => anims.idle,
            PlayerState::Running => anims.run,
        };

        if current.index == Some(target) {
            continue;
        }
        anim_player.stop_all();
        
        anim_player.play(target).repeat();
        current.index = Some(target);
        println!("current anim: {:?}", current.index);
    }
}
