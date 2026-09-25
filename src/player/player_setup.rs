use avian3d::collision::collider::Collider;
use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::combat::autoattack::AutoAttack;
use crate::combat::combat_stats::CombatStats;
use crate::combat::spell_cast::SpellCooldowns;
use crate::player::player_animation::CurrentAnim;
use crate::player::player_update::{
    CoyoteTime, GroundContact, GroundDetection, PlayerController, PlayerMovementSettings,
};
use crate::systems::animation_names::AnimationNames;
use crate::systems::health::Health;
use crate::systems::mana::Mana;

#[derive(Component, Clone, Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Running,
    Jumping,
    Fighting,
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
#[derive(Bundle)]
struct PlayerBundle {
    world_asset: WorldAssetRoot,
    transform: Transform,
    player: Player,
    entity: EntityAnimations,
    anim: CurrentAnim,
    health: Health,
    coyote: CoyoteTime,
    mana: Mana,
    combatstats: CombatStats,
    spellbook_list: SpellBook,
    spellcooldowns: SpellCooldowns,
    player_controller: PlayerController,
    player_movement_settings: PlayerMovementSettings,
    ground_detection: GroundDetection,
    ground_contact: GroundContact,
    linearvelocity: LinearVelocity,
    collider: Collider,
    attack: AutoAttack,
}

pub fn setup_player(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let base = "models/Sword/Sword";
    let graph_handle: Handle<AnimationGraph> = asset_server.load(format!("{base}.animgraph.ron"));
    let names: Handle<AnimationNames> = asset_server.load(format!("{base}.animnames.ron"));
    let mut spellbook = SpellBook::default();
    spellbook.learn("Fireball");
    spellbook.learn("Heal");
    commands.spawn(PlayerBundle {
        world_asset: WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(format!("{base}.gltf"))),
        ),
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        player: Player {
            pos: Vec3::ZERO,
            state: PlayerState::Idle,
        },
        entity: EntityAnimations {
            names,
            graph: graph_handle.clone(),
        },
        anim: CurrentAnim { index: None },
        health: Health::new(100.0),
        coyote: CoyoteTime::default(),
        mana: Mana::new(100.0),
        combatstats: CombatStats {
            attack_speed: 1.0,
            attack_range: 10.0,
            attack_power: 20.0,
            armor: 20.0,
            fire_resist: 20.0,
            frost_resist: 20.0,
            magic_resist: 20.0,
            crit_chance: 0.2,
            crit_multiplier: 2.0,
        },
        spellbook_list: spellbook,
        spellcooldowns: SpellCooldowns::default(),
        player_controller: PlayerController,
        player_movement_settings: PlayerMovementSettings::default(),
        ground_detection: GroundDetection::default(),
        linearvelocity: LinearVelocity::default(),
        collider: Collider::compound(vec![(
            Vec3::new(0.0, 0.9, 0.1),
            Quat::IDENTITY,
            Collider::capsule(0.35, 1.0),
        )]),
        ground_contact: GroundContact::default(),
        attack: AutoAttack {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            active: false,
        },
    });
}
