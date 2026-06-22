use bevy::prelude::*;

use crate::combat::combat_stats::CombatStats;
use crate::combat::spell_cast::SpellCooldowns;
use crate::player::update_player_animation::CurrentAnim;
use crate::systems::health::Health;
use crate::systems::mana::Mana;

#[derive(Component, Clone, Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Running,
}
#[derive(Debug, Component, Default)]
pub struct SpellBook {
    pub spells: Vec<String>,
}
impl SpellBook {
    pub fn learn(&mut self, ability_name: impl Into<String>) {
        let name = ability_name.into();
        if !self.spells.contains(&name) {
            self.spells.push(name);
        }
    }
}
#[derive(Component)]
pub struct PlayerAnimations {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct Player {
    pub pos: Vec3,
    pub state: PlayerState,
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
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
    let graph_handle = graphs.add(graph);
    let mut spellbook = SpellBook::default();
    spellbook.learn("Fireball");
    spellbook.learn("Heal");
    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("mannequiny-0.4.0.gltf")),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            pos: Vec3::ZERO,
            state: PlayerState::Idle,
        },
        PlayerAnimations {
            idle,
            walk,
            run,
            graph: graph_handle.clone(),
        },
        CurrentAnim { index: None },
        AnimationGraphHandle(graph_handle),
        Health::new(100.0),
        Mana::new(100.0),
        spellbook,
        SpellCooldowns::default(),
        CombatStats::new(25.0, 10.0, 2.0, 2.0, 2.0, 0.15, 2.0),
    ));
}
