use crate::combat::attack::attack_system;
use crate::combat::autoattack::{auto_attack_system, toggle_auto_attack};
use crate::combat::spell_cast::{cast_from_input, on_cast, tick_cooldowns};
use crate::core::gamestates::GameState;
use crate::enemies::enemy_test::{propagate_enemy_outlines, spawn_enemy};
use crate::menu::menu_ui::{
    MenuState, button_system, controls_menu_setup, main_menu_setup, menu_action, menu_ui,
    rebind_button_click, refresh_rebind_labels, settings_menu_setup,
};
use crate::player::player_setup::setup_player;
use crate::player::player_update::{
    apply_gravity, move_and_slide_player, update_coyote_time, update_grounded, update_player,
};

use crate::player::player_animation::{setup_player_anim, update_player_animation};
use crate::systems::camera::camera_follow_system;
use crate::systems::damages::{damage_system, test_damages};
use crate::systems::death::death_system;
use crate::systems::heal::heal_system;
use crate::systems::keys::{Rebinding, capture_rebind};
use crate::systems::load_abilities::debug_abilities;
use crate::systems::target::{
    CurrentTarget, clear_target_on_miss, highlight_target, on_enemy_clicked,
};
use crate::systems::{
    camera::setup_camera,
    keys::{keys_setup, keys_update},
    testmap::testmap,
};
use crate::ui::player_ui::{spawn_player_stats_ui, update_bars};
use bevy::prelude::*;

pub struct PlayerSystemPlugin;
impl Plugin for PlayerSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (setup_player, spawn_enemy))
            .add_systems(
                Update,
                (
                    update_player_animation,
                    setup_player_anim.after(setup_player),
                    propagate_enemy_outlines,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                FixedUpdate,
                (
                    update_grounded,
                    apply_gravity,
                    update_player,
                    move_and_slide_player,
                    update_coyote_time,
                )
                    .chain()
                    .run_if(in_state(GameState::Game)),
            );
    }
}
pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTarget>()
            .add_systems(Startup, keys_setup)
            .add_systems(
                OnEnter(GameState::Game),
                (debug_abilities, setup_camera, testmap),
            )
            .add_systems(
                Update,
                (
                    camera_follow_system,
                    keys_update,
                    test_damages,
                    highlight_target,
                    clear_target_on_miss,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_observer(damage_system)
            .add_observer(heal_system)
            .add_observer(death_system);
    }
}

pub struct CombatSystemPlugin;
impl Plugin for CombatSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_enemy_clicked, cast_from_input, tick_cooldowns).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (toggle_auto_attack, auto_attack_system)
                .chain()
                .run_if(in_state(GameState::Game)),
        )
        .add_observer(on_cast)
        .add_observer(attack_system);
    }
}

pub struct PlayerUiSystem;
impl Plugin for PlayerUiSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_player_stats_ui)
            .add_systems(Update, update_bars.run_if(in_state(GameState::Game)));
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_ui) // pass MenuState to Main
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(OnEnter(MenuState::SettingsControls), controls_menu_setup)
            .add_systems(
                OnExit(MenuState::SettingsControls),
                |mut r: ResMut<Rebinding>| r.0 = None,
            )
            .add_systems(
                Update,
                (
                    menu_action,
                    button_system,
                    rebind_button_click,
                    refresh_rebind_labels,
                )
                    .run_if(not(in_state(MenuState::Disabled))),
            )
            .add_systems(Update, capture_rebind);
    }
}
