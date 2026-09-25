use bevy::{
    app::AppExit,
    color::palettes::{
        css::{BLUE, CRIMSON, GREEN, RED},
        tailwind::CYAN_100,
    },
    prelude::*,
};

use crate::{
    core::gamestates::GameState,
    systems::keys::{Action, Bindings, KeysConfig, Rebinding, key_label, save_config},
};
pub fn menu_ui(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsControls,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    SettingsControls,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    ResetKeys,
    BackToSettings,
    Quit,
}
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn main_menu_setup(mut commands: Commands) {
    // Common style for all buttons on the screen
    debug!("Starting menu init");
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: px(30),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: px(10),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: FontSize::Px(33.0),
        ..default()
    };

    /*   let right_icon = asset_server.load("textures/Game Icons/right.png");

        let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
        let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");
    */
    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Camera2d::default(),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // Display the game name
                (
                    Text::new("Dungeons"),
                    TextFont {
                        font_size: FontSize::Px(67.0),
                        ..default()
                    },
                    TextColor(Color::Srgba(CYAN_100)),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),
                // Display three buttons for each action available from the main menu:
                // - new game
                // - settings
                // - quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        //(ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(Color::Srgba(GREEN)),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        //(ImageNode::new(wrench_icon), button_icon_node.clone()),
                        (
                            Text::new("Settings"),
                            button_text_font.clone(),
                            TextColor(Color::Srgba(BLUE)),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        //(ImageNode::new(exit_icon), button_icon_node),
                        (
                            Text::new("Quit"),
                            button_text_font,
                            TextColor(Color::Srgba(RED)),
                        ),
                    ]
                ),
            ]
        )],
    ));
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut config: ResMut<KeysConfig>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::SettingsControls => menu_state.set(MenuState::SettingsControls),
                MenuButtonAction::ResetKeys => {
                    *config = KeysConfig::default();
                    save_config(&config);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}
pub fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(10)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let font = TextFont {
        font_size: FontSize::Px(30.0),
        ..default()
    };

    commands.spawn((
        DespawnOnExit(MenuState::Settings),
        Camera2d::default(),
        OnSettingsMenuScreen,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::SettingsControls,
                    children![(Text::new("Controls"), font.clone())]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::SettingsDisplay,
                    children![(Text::new("Display"), font.clone())]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::SettingsSound,
                    children![(Text::new("Sound"), font.clone())]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToMainMenu,
                    children![(Text::new("Back"), font)]
                ),
            ]
        )],
    ));
}
#[derive(Component)]
struct OnControlsMenuScreen;

#[derive(Component)]
pub struct RebindButton(pub Action, pub usize);

pub fn controls_menu_setup(mut commands: Commands) {
    let font = TextFont {
        font_size: FontSize::Px(20.0),
        ..default()
    };
    let key_button = Node {
        width: px(160),
        height: px(34),
        margin: UiRect::all(px(3)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            DespawnOnExit(MenuState::SettingsControls),
            Camera2d::default(),
            OnControlsMenuScreen,
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(px(20)),
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
            ))
            .with_children(|panel| {
                for &action in Action::ALL {
                    panel
                        .spawn(Node {
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            width: px(420),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((Text::new(action.pretty()), font.clone()));
                            row.spawn(Node::default()).with_children(|keys| {
                                for slot in 0..2 {
                                    keys.spawn((
                                        Button,
                                        key_button.clone(),
                                        BackgroundColor(NORMAL_BUTTON),
                                        RebindButton(action, slot),
                                    ))
                                    .with_child((Text::new(""), font.clone()));
                                }
                            });
                        });
                }

                panel
                    .spawn(Node {
                        justify_content: JustifyContent::Center,
                        margin: UiRect::top(px(15)),
                        ..default()
                    })
                    .with_children(|row| {
                        for (label, act) in [
                            ("Reset", MenuButtonAction::ResetKeys),
                            ("Back", MenuButtonAction::BackToSettings),
                        ] {
                            row.spawn((
                                Button,
                                Node {
                                    width: px(160),
                                    height: px(45),
                                    margin: UiRect::all(px(6)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(NORMAL_BUTTON),
                                act,
                            ))
                            .with_child((Text::new(label), font.clone()));
                        }
                    });
            });
        });
}

pub fn rebind_button_click(
    q: Query<(&Interaction, &RebindButton), Changed<Interaction>>,
    mut rebinding: ResMut<Rebinding>,
) {
    for (interaction, RebindButton(action, slot)) in &q {
        if *interaction == Interaction::Pressed {
            rebinding.0 = Some((*action, *slot));
        }
    }
}

pub fn refresh_rebind_labels(
    config: Res<KeysConfig>,
    rebinding: Res<Rebinding>,
    buttons: Query<(&RebindButton, &Children)>,
    mut texts: Query<&mut Text>,
) {
    for (RebindButton(action, slot), children) in &buttons {
        let label = if rebinding.0 == Some((*action, *slot)) {
            "Press a key...".to_string()
        } else {
            match config.get(*action).get(*slot) {
                Some(Bindings::KeyCode(k)) => key_label(*k),
                Some(Bindings::MouseButton(m)) => format!("{:?}", m),
                None => "-".to_string(),
            }
        };
        for child in children.iter() {
            if let Ok(mut t) = texts.get_mut(child) {
                if t.0 != label {
                    t.0 = label.clone();
                }
            }
        }
    }
}
