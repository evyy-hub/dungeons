use bevy::{
    input::{
        ButtonState,
        keyboard::KeyboardInput,
        mouse::{MouseButton, MouseButtonInput},
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    get_single_component, get_single_mut_component, resources::path::keys_config_path,
    utils::file_manager::open_or_create_toml,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Bindings {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
}

macro_rules! define_keys {
    ($($field:ident),*) => {
        #[derive(Debug, Component, Default)]
        pub struct Actions {
            $( pub $field: bool, )*
        }

        #[derive(Debug, Component, Default, Serialize, Deserialize, Clone)]
        pub struct KeysConfig {
            $( pub $field: Vec<Bindings>, )*
        }

        impl Actions {
            fn update_from_input(
                &mut self,
                config: &KeysConfig,
                kbd_ev: Option<&KeyboardInput>,
                mouse_ev: Option<&MouseButtonInput>,
            ) {
                $(
                    self.$field = update_action(&config.$field, kbd_ev, mouse_ev, self.$field);
                )*
            }
        }
    }
}

define_keys!(
    move_left,
    move_right,
    move_up,
    move_down,
    exit,
    interact,
    option,
    camera_mode,
    new_action_without_binding
);

fn update_action(
    bindings: &[Bindings],
    kbd: Option<&KeyboardInput>,
    mouse: Option<&MouseButtonInput>,
    current: bool,
) -> bool {
    let mut new_state = current;
    for binding in bindings {
        if let Some(kbd) = kbd {
            if let Bindings::KeyCode(key_code) = binding {
                match kbd.state {
                    ButtonState::Pressed if &kbd.key_code == key_code => new_state = true,
                    ButtonState::Released if &kbd.key_code == key_code => new_state = false,
                    _ => {}
                }
            }
        }
        if let Some(mouse) = mouse {
            if let Bindings::MouseButton(btn) = binding {
                match mouse.state {
                    ButtonState::Pressed if &mouse.button == btn => {
                        new_state = true;
                    }
                    ButtonState::Released if &mouse.button == btn => {
                        new_state = false;
                    }
                    _ => {}
                }
            }
        }
    }
    new_state
}

pub fn keys_setup(mut commands: Commands) {
    commands.spawn(Actions::default());

    let default_config = KeysConfig {
        move_right: vec![
            Bindings::KeyCode(KeyCode::KeyD),
            Bindings::KeyCode(KeyCode::ArrowRight),
        ],
        move_left: vec![
            Bindings::KeyCode(KeyCode::KeyA),
            Bindings::KeyCode(KeyCode::ArrowLeft),
        ],
        move_up: vec![
            Bindings::KeyCode(KeyCode::KeyW),
            Bindings::KeyCode(KeyCode::ArrowUp),
        ],
        move_down: vec![
            Bindings::KeyCode(KeyCode::KeyS),
            Bindings::KeyCode(KeyCode::ArrowDown),
        ],
        exit: vec![Bindings::KeyCode(KeyCode::Escape)],
        interact: vec![
            Bindings::MouseButton(MouseButton::Left),
            Bindings::MouseButton(MouseButton::Right),
        ],
        ..Default::default()
    };

    let config: KeysConfig = open_or_create_toml(&keys_config_path(), default_config);

    commands.spawn(config);
}

pub fn keys_update(
    mut evr_kbd: MessageReader<KeyboardInput>,
    mut evr_mouse: MessageReader<MouseButtonInput>,
    mut actions_query: Query<&mut Actions>,
    keys_config_query: Query<&KeysConfig>,
) {
    let mut actions = get_single_mut_component!(actions_query);
    let config = get_single_component!(keys_config_query);

    for kbd_ev in evr_kbd.read() {
        actions.update_from_input(config, Some(kbd_ev), None);
    }

    for mouse_ev in evr_mouse.read() {
        actions.update_from_input(config, None, Some(mouse_ev));
    }
   // println!("{:#?}", *actions);
}
