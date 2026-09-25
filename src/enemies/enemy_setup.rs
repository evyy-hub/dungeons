use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_outline::OutlineVolume;

use crate::{
    combat::combat_stats::CombatStats,
    enemies::enemy_test::GameLayer,
    player::player_setup::EntityAnimations,
    systems::{animation_names::AnimationNames, health::Health},
};

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemy_model(
    commands: &mut Commands,
    asset_server: &AssetServer,
    model_name: &str,
    transform: Transform,
) -> Entity {
    let base = format!("models/{model_name}/{model_name}");

    let scene = asset_server.load(format!("{base}.gltf#Scene0"));
    let graph: Handle<AnimationGraph> = asset_server.load(format!("{base}.animgraph.ron"));
    let names: Handle<AnimationNames> = asset_server.load(format!("{base}.animnames.ron"));
    // let stats: Handle<Stats> = asset_server.load(format!("{base}.stats.ron"));

    commands
        .spawn((
            WorldAssetRoot(scene),
            transform,
            Enemy {},
            EntityAnimations { names, graph },
            OutlineVolume {
                visible: false,
                colour: Color::srgb(1.0, 0.8, 0.0),
                width: 2.0,
            },
            CombatStats::new(15.0,10.0, 20.0, 20.0, 2.0, 2.0, 5.0, 2.0, 1.5),
            Health::new(300.0),
        ))
        .with_child((
            Collider::capsule(0.4, 1.0),
            CollisionLayers::new(
                GameLayer::Enemy,
                [GameLayer::Player, GameLayer::PlayerSpell],
            ),
            Transform::from_xyz(0.0, 0.9, 0.1),
        ))
        .id()
}
