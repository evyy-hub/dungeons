use avian3d::
    PhysicsPlugins 
;
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::*;
use bevy_mod_outline::{AutoGenerateOutlineNormalsPlugin, OutlinePlugin};
use dungeons::{
    core::gamestates::GameState,
    plugins_def::{
        CombatSystemPlugin, GameSystemsPlugin, MenuPlugin, PlayerSystemPlugin, PlayerUiSystem,
    },
    systems::{animation_names::AnimationNamesPlugin, load_abilities::AbilitiesConfig},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dungeons".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_plugins((

            PhysicsPlugins::default(),

            //  PhysicsDebugPlugin::default(),
            OutlinePlugin::EXTRUDE_VERTEX,
            AutoGenerateOutlineNormalsPlugin::default(),
            MeshPickingPlugin,
            GameSystemsPlugin,
            PlayerSystemPlugin,
            PlayerUiSystem,
            MenuPlugin,
            AnimationNamesPlugin,
            CombatSystemPlugin,
        ))
        .insert_resource(AbilitiesConfig::default())
        .run();
}
