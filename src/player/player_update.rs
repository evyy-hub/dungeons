use crate::player::player_setup::PlayerState;
use crate::systems::camera::FollowCamera;
use crate::systems::death::Dead;
use crate::{player::player_setup::Player, systems::keys::Actions};
use bevy::prelude::*;

pub fn update_player(
    mut player_q: Query<(&mut Transform, &mut Player), Without<Dead>>,
    action_q: Query<&Actions>,
    camera_q: Query<(&FollowCamera, &Transform), Without<Player>>,

    time: Res<Time>,
) {
    let delta = time.delta_secs();

    let (mut transform, mut player) = match player_q.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let actions = match action_q.single() {
        Ok(a) => a,
        Err(_) => return,
    };

    let (_camera, cam_tf) = match camera_q.single() {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut forward = Vec3::from(cam_tf.forward());

    forward.y = 0.0;
    forward = forward.normalize();

    let right = Vec3::new(-forward.z, 0.0, forward.x);

    let mut movement = Vec3::ZERO;

    if actions.move_up {
        movement += forward;
    }
    if actions.move_down {
        movement -= forward;
    }
    if actions.move_left {
        movement -= right;
    }
    if actions.move_right {
        movement += right;
    }

    if movement != Vec3::ZERO {
        movement = movement.normalize();

        let speed = 6.0;
        transform.translation += movement * speed * delta;

        let target_rot = Quat::from_rotation_y((movement.x).atan2(movement.z));

        transform.rotation = transform.rotation.slerp(target_rot, 10.0 * delta);
    }
    let moving = actions.move_up || actions.move_down || actions.move_left || actions.move_right;

    player.state = if moving {
        PlayerState::Running
    } else {
        PlayerState::Idle
    };
}
