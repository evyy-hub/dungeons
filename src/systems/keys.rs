use bevy::{
    input::{
        ButtonState,
        keyboard::KeyboardInput,
        mouse::{MouseButton, MouseButtonInput},
    },
    platform::collections::HashSet,
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{resources::path::keys_config_path, utils::file_manager::open_or_create_toml};
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Bindings {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
}

macro_rules! define_keys {
    ($($field:ident => $variant:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Action { $($variant),* }

        impl Action {
            pub const ALL: &'static [Action] = &[$(Action::$variant),*];
            pub fn label(&self) -> &'static str {
                match self { $(Action::$variant => stringify!($field)),* }
            }
        }
        impl Action {
            pub fn pretty(&self) -> &'static str {
        match self {
            Action::MoveLeft => "Move left",
            Action::MoveRight => "Move right",
            Action::MoveUp => "Move up",
            Action::MoveDown => "Move down",
            Action::Jump => "Jump",
            Action::Exit => "Exit",
            Action::Spell1 => "Spell 1",
            Action::Spell2 => "Spell 2",
            Action::Spell3 => "Spell 3",
            Action::Spell4 => "Spell 4",
            Action::ToggleAutoAttack => "Auto attack",
        }
    }
}

        #[derive(Resource, Serialize, Deserialize, Clone)]
        pub struct KeysConfig { $(pub $field: Vec<Bindings>),* }

        impl KeysConfig {
            pub fn get(&self, a: Action) -> &Vec<Bindings> {
                match a { $(Action::$variant => &self.$field),* }
            }
            pub fn get_mut(&mut self, a: Action) -> &mut Vec<Bindings> {
                match a { $(Action::$variant => &mut self.$field),* }
            }
        }
    };
}

define_keys!(
    move_left => MoveLeft, move_right => MoveRight,
    move_up => MoveUp, move_down => MoveDown,
    jump => Jump, exit => Exit,
    spell_1 => Spell1, spell_2 => Spell2, spell_3 => Spell3, spell_4 => Spell4,
    toggle_auto_attack => ToggleAutoAttack,
);
fn keys(list: &[KeyCode]) -> Vec<Bindings> {
    list.iter().map(|k| Bindings::KeyCode(*k)).collect()
}

impl Default for KeysConfig {
    fn default() -> Self {
        Self {
            move_left: keys(&[KeyCode::KeyA, KeyCode::ArrowLeft]),
            move_right: keys(&[KeyCode::KeyD, KeyCode::ArrowRight]),
            move_up: keys(&[KeyCode::KeyW, KeyCode::ArrowUp]),
            move_down: keys(&[KeyCode::KeyS, KeyCode::ArrowDown]),
            jump: keys(&[KeyCode::Space]),
            exit: keys(&[KeyCode::Escape]),
            spell_1: keys(&[KeyCode::KeyJ]),
            spell_2: keys(&[KeyCode::KeyK]),
            spell_3: keys(&[KeyCode::KeyL]),
            spell_4: keys(&[KeyCode::KeyM]),
            toggle_auto_attack: keys(&[KeyCode::Digit1]),
        }
    }
}
pub fn keys_setup(mut commands: Commands) {
    let config: KeysConfig = open_or_create_toml(&keys_config_path(), KeysConfig::default());
    commands.insert_resource(config);
    commands.init_resource::<Actions>();
    commands.init_resource::<Rebinding>();
}
#[derive(Resource, Default)]
pub struct Actions {
    pressed: HashSet<Action>,
    just_pressed: HashSet<Action>,
}

impl Actions {
    pub fn pressed(&self, a: Action) -> bool {
        self.pressed.contains(&a)
    }
    pub fn just_pressed(&self, a: Action) -> bool {
        self.just_pressed.contains(&a)
    }
}

pub fn keys_update(
    kb: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<KeysConfig>,
    mut actions: ResMut<Actions>,
) {
    actions.pressed.clear();
    actions.just_pressed.clear();

    for &a in Action::ALL {
        for b in config.get(a) {
            let (p, jp) = match b {
                Bindings::KeyCode(k) => (kb.pressed(*k), kb.just_pressed(*k)),
                Bindings::MouseButton(m) => (mouse.pressed(*m), mouse.just_pressed(*m)),
            };
            if p {
                actions.pressed.insert(a);
            }
            if jp {
                actions.just_pressed.insert(a);
            }
        }
    }
}
#[derive(Resource, Default)]
pub struct Rebinding(pub Option<(Action, usize)>); // (action, index binding)

pub fn capture_rebind(
    mut rebinding: ResMut<Rebinding>,
    mut kbd: MessageReader<KeyboardInput>,
    mut mouse: MessageReader<MouseButtonInput>,
    mut config: ResMut<KeysConfig>,
) {
    let Some((action, slot)) = rebinding.0 else {
        kbd.clear();
        mouse.clear();
        return;
    };

    let mut new_binding = None;
    let mut cancel = false;

    for ev in kbd.read() {
        info!("[REBIND] {:?} {:?}", ev.key_code, ev.state);
        if ev.state != ButtonState::Pressed {
            continue;
        }
        if ev.key_code == KeyCode::Escape {
            cancel = true;
        } else {
            new_binding = Some(Bindings::KeyCode(ev.key_code));
        }
    }

    if cancel {
        rebinding.0 = None;
        return;
    }

    if let Some(b) = new_binding {
        for &a in Action::ALL {
            config.get_mut(a).retain(|x| !same_binding(x, &b));
        }
        let list = config.get_mut(action);
        if slot < list.len() {
            list[slot] = b;
        } else {
            list.push(b);
        }
        rebinding.0 = None;
        save_config(&config);
    }
}

fn same_binding(a: &Bindings, b: &Bindings) -> bool {
    match (a, b) {
        (Bindings::KeyCode(x), Bindings::KeyCode(y)) => x == y,
        (Bindings::MouseButton(x), Bindings::MouseButton(y)) => x == y,
        _ => false,
    }
}

pub fn save_config(config: &KeysConfig) {
    if let Ok(s) = toml::to_string_pretty(config) {
        let _ = std::fs::write(keys_config_path(), s);
    }
}
pub fn reset_keys(mut config: ResMut<KeysConfig>) {
    *config = KeysConfig::default();
    save_config(&config);
}

pub fn key_label(key: KeyCode) -> String {
    let s = format!("{:?}", key);
    s.strip_prefix("Key")
        .or_else(|| s.strip_prefix("Digit"))
        .or_else(|| s.strip_prefix("Numpad"))
        .unwrap_or(&s)
        .to_string()
}
