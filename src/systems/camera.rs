use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::player::player_setup::Player;

#[derive(Component)]
pub struct FollowCamera {
    pub(crate) yaw: f32,
    pitch: f32,
    distance: f32,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(DirectionalLight {
        illuminance: 5000.0,

        ..Default::default()
    });

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        FollowCamera {
            yaw: 0.0,
            pitch: 0.45,
            distance: 10.0,
        },
    ));
}
pub fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut FollowCamera, &mut Transform), Without<Player>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut windows: Query<(&mut Window, &mut CursorOptions), With<PrimaryWindow>>,
) {
    let player_tf = match player_query.single() {
        Ok(p) => p,
        Err(_) => return,
    };

    let (mut follow, mut cam_tf) = match camera_query.single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };
    let Ok((mut _window, mut cursor)) = windows.single_mut() else {
        return;
    };

    // Rotation uniquement si clic droit
    if mouse_buttons.pressed(MouseButton::Right) {
        for event in mouse_motion.read() {
            follow.yaw -= event.delta.x * 0.005;
            follow.pitch -= event.delta.y * 0.005;
        }
    }

    // Clamp vertical
    follow.pitch = follow.pitch.clamp(-1.4, 1.4);

    // Zoom molette
    for scroll in mouse_wheel.read() {
        follow.distance -= scroll.y * 0.5;
    }

    follow.distance = follow.distance.clamp(3.0, 25.0);

    // Calcul position orbit
    let offset = Vec3::new(
        follow.yaw.cos() * follow.pitch.cos() * follow.distance,
        follow.pitch.sin() * follow.distance + 2.0,
        follow.yaw.sin() * follow.pitch.cos() * follow.distance,
    );

    cam_tf.translation = player_tf.translation + offset;
    cam_tf.look_at(player_tf.translation + Vec3::Y * 1.5, Vec3::Y);

    // Lock curseur quand clic droit

    if mouse_buttons.pressed(MouseButton::Right) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    } else {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}
