use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_outline::InheritOutline;

use crate::ennemies::ennemy_setup::spawn_enemy_model;
use crate::player::player_setup::EntityAnimations;
use crate::systems::animation_names::AnimationNames;

#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct OutlineSetup;

#[derive(PhysicsLayer, Default, Clone, Copy, Debug)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Terrain,
    PlayerSpell,
    EnemySpell,
    Hitbox, // temporary attack hitboxes
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_enemy_model(
        &mut commands,
        &asset_server,
        "Adventurer",
        Transform::from_xyz(10.0, 0.0, 0.0),
    );
}

pub fn propagate_enemy_outlines(
    mut commands: Commands,
    enemy_query: Query<&Children, With<Enemy>>,
    children_query: Query<&Children>,
) {
    for children in enemy_query.iter() {
        for child in children.iter() {
            commands.entity(child).insert(InheritOutline);

            // Recursively add InheritOutline to all descendants
            add_inherit_outline_recursive(child, &mut commands, &children_query);
        }
    }
}

fn add_inherit_outline_recursive(
    entity: Entity,
    commands: &mut Commands,
    children_query: &Query<&Children>,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            commands.entity(child).insert(InheritOutline);
            add_inherit_outline_recursive(child, commands, children_query);
        }
    }
}

/// System that initializes the AnimationPlayer once the scene is loaded only for test.
/// The AnimationPlayer is added by Bevy on a child entity when the glTF scene spawns.
pub fn setup_enemy_animations(
    mut _commands: Commands,
    _enemy_query: Query<(Entity, &EntityAnimations), With<Enemy>>,
    _children_query: Query<&Children>,
    _names_assets: Res<Assets<AnimationNames>>,
    mut _players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    /*   for (enemy_entity, anims) in &enemy_query {
        let Some(names) = names_assets.get(&anims.names) else {
            continue; // names not loaded yet, retry next frame
        };
        let Some(idle) = names.try_get("Run") else {
            continue;
        };

        for descendant in children_query.iter_descendants(enemy_entity) {
            if let Ok((player_entity, mut player)) = players.get_mut(descendant) {
                commands
                    .entity(player_entity)
                    .insert(AnimationGraphHandle(anims.graph.clone()));
                player.play(idle).repeat();
            }
        }
    }*/
}
