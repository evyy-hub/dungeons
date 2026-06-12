use avian3d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::*;
use bevy_mod_outline::{AutoGenerateOutlineNormalsPlugin, OutlinePlugin};
use dungeons::{
    plugins_def::{CombatSystemPlugin, GameSystemsPlugin, PlayerSystemPlugin, PlayerUiSystem},
    systems::load_abilities::AbilitiesConfig,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dungeons".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins(OutlinePlugin)
        .add_plugins(AutoGenerateOutlineNormalsPlugin::default())
        .add_plugins(MeshPickingPlugin)
        .add_plugins(GameSystemsPlugin)
        .add_plugins(PlayerSystemPlugin)
        .add_plugins(PlayerUiSystem)
        .insert_resource(AbilitiesConfig::default())
        .add_plugins(CombatSystemPlugin)
        .run();
}
