use crate::combat::attack::attack_system;
use crate::player::enemy_test::{propagate_enemy_outlines, setup_enemy_animations, spawn_enemy};
use crate::player::player_setup::setup_player;
use crate::player::player_update::update_player;

use crate::player::update_player_animation::{setup_player_anim, update_player_animation};
use crate::systems::camera::camera_follow_system;
use crate::systems::damages::{damage_system, test_damages};
use crate::systems::death::death_system;
use crate::systems::target::{
    CurrentTarget, clear_target_on_miss, highlight_target, on_enemy_clicked,
};
use crate::systems::{
    camera::setup_camera,
    keys::{keys_setup, keys_update},
    testmap::testmap,
};
use bevy::prelude::*;
pub struct PlayerSystemPlugin;
impl Plugin for PlayerSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Startup, spawn_enemy)
            .add_systems(Update, propagate_enemy_outlines)
            .add_systems(Update, setup_enemy_animations)
            .add_systems(Update, setup_player_anim.after(setup_player))
            .add_systems(Update, update_player_animation)
            .add_systems(Update, update_player);
    }
}
pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTarget>()
            .add_systems(Startup, testmap)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow_system)
            .add_systems(Startup, keys_setup)
            .add_systems(Update, keys_update)
            .add_systems(Update, test_damages)
            .add_systems(Update, highlight_target)
            .add_systems(Update, clear_target_on_miss)
            .add_observer(damage_system)
            .add_observer(death_system);
    }
}

pub struct CombatSystemPlugin;
impl Plugin for CombatSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attack_system)
            .add_systems(Update, on_enemy_clicked);
    }
}
