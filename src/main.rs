use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::*;
use dungeons::plugins_def::{CombatSystemPlugin, GameSystemsPlugin, PlayerSystemPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dungeons".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MeshPickingPlugin)
        .add_plugins(GameSystemsPlugin)
        .add_plugins(PlayerSystemPlugin)
        .add_plugins(CombatSystemPlugin)
        .run();
}
