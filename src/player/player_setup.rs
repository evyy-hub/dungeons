use bevy::prelude::*;

use crate::combat::combat_stats::CombatStats;
use crate::combat::spell_cast::SpellCooldowns;
use crate::player::player_animation::CurrentAnim;
use crate::systems::animation_names::AnimationNames;
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
pub struct EntityAnimations {
    pub names: Handle<AnimationNames>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct Player {
    pub pos: Vec3,
    pub state: PlayerState,
}

pub fn setup_player(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    
    
    let base = "models/Adventurer/Adventurer"; // adapte au nom réel du modèle joueur
    let graph_handle: Handle<AnimationGraph> = asset_server.load(format!("{base}.animgraph.ron"));
    let names: Handle<AnimationNames> = asset_server.load(format!("{base}.animnames.ron"));
    let mut spellbook = SpellBook::default();
    spellbook.learn("Fireball");
    spellbook.learn("Heal");
    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(format!("{base}.gltf"))),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            pos: Vec3::ZERO,
            state: PlayerState::Idle,
        },
        EntityAnimations {
            names,
            graph: graph_handle.clone(),
        },
        CurrentAnim { index: None },
        Health::new(100.0),
        Mana::new(100.0),
        spellbook,
        SpellCooldowns::default(),
        CombatStats::new(25.0, 10.0, 2.0, 2.0, 2.0, 0.15, 2.0),
    ));
}
