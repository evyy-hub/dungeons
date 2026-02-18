use crate::player::player_setup::setup_player;
use crate::player::player_update::update_player;

use crate::systems::camera::camera_follow_system;
use crate::systems::{
    camera::setup_camera,
    keys::{keys_setup, keys_update},
    testmap::testmap,
};
use bevy::prelude::*;
pub struct PlayerSystemPlugin;
impl Plugin for PlayerSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, update_player);
    }
}
pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, testmap)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow_system)
            .add_systems(Startup, keys_setup)
            .add_systems(Update, keys_update);
    }
}


