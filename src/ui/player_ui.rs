use bevy::prelude::*;

use crate::{
    player::player_setup::Player,
    systems::{health::Health, mana::Mana},
};

#[derive(Component)]
pub struct HpBar;

#[derive(Component)]
pub struct ManaBar;

pub fn spawn_player_stats_ui(mut commands: Commands, _query: Query<(&Health, &Mana), With<Player>>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.),
                left: Val::Px(20.),
                width: Val::Px(200.),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.),
                ..default()
            },
            BackgroundColor(Color::srgba(0., 0., 0., 0.5)),
        ))
        .with_children(|parent| {
            // HP BAR BACKGROUND
            parent
                .spawn(Node {
                    width: Val::Px(200.),
                    height: Val::Px(18.),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    ));

                    p.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                        HpBar,
                    ));
                });

            // MANA BAR BACKGROUND
            parent
                .spawn(Node {
                    width: Val::Px(200.),
                    height: Val::Px(18.),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    ));

                    p.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.4, 0.9)),
                        ManaBar,
                    ));
                });
        });
}
pub fn update_bars(
    player_q: Query<(&Health, &Mana), With<Player>>,
    mut bars_q: Query<(&mut Node, Option<&HpBar>, Option<&ManaBar>)>,
) {
    let Ok((health, mana)) = player_q.single() else {
        return;
    };

    let hp_ratio = health.get_current() / health.get_max();
    let mana_ratio = mana.get_current() / mana.get_max();

    for (mut node, hp, mana_bar) in &mut bars_q {
        if hp.is_some() {
            node.width = Val::Percent(hp_ratio * 100.0);
        } else if mana_bar.is_some() {
            node.width = Val::Percent(mana_ratio * 100.0);
        }
    }
}
