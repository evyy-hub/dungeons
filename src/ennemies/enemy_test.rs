use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_outline::{InheritOutline, OutlineVolume};

use crate::combat::combat_stats::CombatStats;
use crate::systems::health::Health;

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
/// Stores animation node indices for an enemy's animations.
#[derive(Component)]
pub struct EnemyAnimations {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub fight_idle: AnimationNodeIndex,
    pub fight_punch: AnimationNodeIndex,
    pub fight_kick: AnimationNodeIndex,
    pub graph: Handle<AnimationGraph>,
}

pub fn spawn_enemy(
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut graph = AnimationGraph::new();

    let idle = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(7).from_asset("mannequiny-0.4.0.gltf")),
        1.0,
        graph.root,
    );
    let walk = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(10).from_asset("mannequiny-0.4.0.gltf")),
        1.0,
        graph.root,
    );
    let run = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(9).from_asset("mannequiny-0.4.0.gltf")),
        1.0,
        graph.root,
    );
    let fight_idle = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(4).from_asset("mannequiny-0.4.0.gltf")),
        1.0,
        graph.root,
    );
    let fight_punch = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(6).from_asset("mannequiny-0.4.0.gltf")),
        1.0,
        graph.root,
    );
    let fight_kick = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(5).from_asset("mannequiny-0.4.0.gltf")),
        1.0,
        graph.root,
    );

    let graph_handle = graphs.add(graph);

    commands
        .spawn((
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("mannequiny-0.4.0.gltf")),
            ),
            Transform::from_xyz(30.0, 0.0, 0.0),
            Enemy {},
            Health::new(300.0),
            CombatStats::new(15.0, 20.0, 0.05, 2.0, 2.0, 5.0, 2.0),
            EnemyAnimations {
                idle,
                walk,
                run,
                fight_idle,
                fight_punch,
                fight_kick,
                graph: graph_handle.clone(),
            },
            OutlineVolume {
                visible: false,
                colour: Color::srgb(1.0, 0.8, 0.0),
                width: 2.0,
            },
        ))
        .with_child((
            Collider::capsule(0.4, 1.0),
            CollisionLayers::new(
                GameLayer::Enemy,
                [GameLayer::Player, GameLayer::PlayerSpell],
            ),
            Transform::from_xyz(0.0, 0.9, 0.5),
        ));

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            })),
            Transform::from_xyz(10.0, 1.0, 0.0),
            Enemy {},
            Health::new(1000.0),
            CombatStats::new(15.0, 20.0, 0.05, 2.0, 2.0, 5.0, 2.0),
            OutlineVolume {
                visible: false,
                colour: Color::srgb(1.0, 0.8, 0.0),
                width: 2.0,
            },
        ))
        .with_child((
            // Child entity collider with local offset to preserve model position
            Collider::cuboid(0.4, 1.0, 0.4),
            CollisionLayers::new(
                GameLayer::Enemy,
                [GameLayer::Player, GameLayer::PlayerSpell],
            ),
            Transform::from_xyz(0.0, 0.9, 0.5),
        ));
    commands
        .spawn((
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("mannequiny-0.4.0.gltf")),
            ),
            Enemy {},
            Health::new(300.0),
            CombatStats::new(15.0, 20.0, 0.05, 2.0, 2.0, 5.0, 2.0),
            EnemyAnimations {
                idle,
                walk,
                run,
                fight_idle,
                fight_punch,
                fight_kick,
                graph: graph_handle.clone(),
            },
            RigidBody::Kinematic,
            Transform::from_xyz(15.0, 0.0, 0.0),
            OutlineVolume {
                visible: false,
                colour: Color::srgb(1.0, 0.8, 0.0),
                width: 2.0,
            },
        ))
        .with_child((
            Collider::capsule(0.4, 1.0),
            CollisionLayers::new(
                GameLayer::Enemy,
                [GameLayer::Player, GameLayer::PlayerSpell],
            ),
            Transform::from_xyz(0.0, 0.9, 0.5),
        ));
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

/// System that initializes the AnimationPlayer once the scene is loaded.
/// The AnimationPlayer is added by Bevy on a child entity when the glTF scene spawns.
pub fn setup_enemy_animations(
    mut commands: Commands,
    enemy_query: Query<&EnemyAnimations, With<Enemy>>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for anims in &enemy_query {
        for (entity, mut player) in &mut players {
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(anims.graph.clone()));
            player.play(anims.run).repeat();
        }
    }
}
