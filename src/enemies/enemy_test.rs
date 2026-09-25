use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_outline::InheritOutline;

use crate::enemies::enemy_setup::{Enemy, spawn_enemy_model};

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
        "Sword",
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
