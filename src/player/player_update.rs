use crate::player::player_setup::PlayerState;
use crate::systems::camera::FollowCamera;
use crate::systems::death::Dead;
use crate::systems::keys::Action;
use crate::{player::player_setup::Player, systems::keys::Actions};
use avian3d::{math::*, prelude::*};
use bevy::gizmos::gizmos::Gizmos;
use bevy::prelude::*;

#[derive(Component)]
#[require(RigidBody::Kinematic, CustomPositionIntegration)]
pub struct PlayerController;

#[derive(Component, Default)]
pub struct CoyoteTime {
    timer: f32,
}
const COYOTE_DURATION: f32 = 0.01; // 150ms

#[derive(Component, Default)]
pub struct GroundContact {
    pub touching_ground: bool,
}

#[derive(Component)]
pub struct PlayerMovementSettings {
    pub speed: f32,
    pub gravity: Vec3,
    pub terminal_velocity: f32,
    pub jump_impulse: f32,
}
impl Default for PlayerMovementSettings {
    fn default() -> Self {
        Self {
            speed: 6.0,
            gravity: Vec3::new(0.0, -9.81 * 2.0, 0.0),
            terminal_velocity: 50.0,
            jump_impulse: 8.5,
        }
    }
}
#[derive(Component)]
pub struct GroundDetection {
    pub max_angle: f32,
    pub max_distance: f32,
    pub cast_shape: Collider,
    pub sphere_radius: f32,
    origin_offset: Vec3,
}
impl Default for GroundDetection {
    fn default() -> Self {
        let radius = 0.2;
        Self {
            max_angle: PI / 6.0,
            max_distance: 0.5,
            sphere_radius: radius,
            cast_shape: Collider::sphere(radius),
            origin_offset: Vec3::new(0.0, 0.3, 0.0),
        }
    }
}
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

pub fn update_grounded(
    mut commands: Commands,
    query: Query<(Entity, &GroundDetection, &GlobalTransform, &GroundContact)>,
    spatial_query: SpatialQuery,
    mut gizmos: Gizmos,
) {
    for (entity, ground, global_tf, contact) in &query {
        let origin = global_tf.translation() + ground.origin_offset;
        let rotation = global_tf.rotation();

        let hit = spatial_query.cast_shape(
            &ground.cast_shape,
            origin,
            rotation,
            global_tf.down(),
            &ShapeCastConfig::from_max_distance(ground.max_distance),
            &SpatialQueryFilter::from_excluded_entities([entity]),
        );
        // println!("Cast from {:?}, hit = {:?}", origin, hit.is_some());
        let cast_grounded = hit.is_some_and(|hit| {
            let up = global_tf.up();
            (rotation * hit.normal1).angle_between(*up) <= ground.max_angle
        });
        let is_grounded = cast_grounded || contact.touching_ground;

        let color = if is_grounded {
            Color::srgb(0.0, 1.0, 0.0)
        } else {
            Color::srgb(1.0, 0.0, 0.0)
        };

        gizmos.sphere(origin, ground.sphere_radius, color);

        let end = origin + *global_tf.down() * ground.max_distance;
        gizmos.line(origin, end, color);

        if let Some(hit) = &hit {
            let hit_point = origin + *global_tf.down() * hit.distance;
            gizmos.sphere(hit_point, ground.sphere_radius, Color::srgb(1.0, 1.0, 0.0));
        }
        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}
pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&PlayerMovementSettings, &mut LinearVelocity)>,
) {
    let delta = time.delta_secs();
    for (settings, mut velocity) in &mut query {
        let gravity_dir = settings.gravity.normalize_or_zero();
        let vel_along_gravity = velocity.dot(gravity_dir);
        if vel_along_gravity < settings.terminal_velocity {
            velocity.0 += settings.gravity * delta;
        }
    }
}

pub fn update_player(
    mut player_q: Query<
        (
            &PlayerMovementSettings,
            &mut Transform,
            &mut LinearVelocity,
            &mut Player,
            Has<Grounded>,
            &mut CoyoteTime,
        ),
        Without<Dead>,
    >,
    action_q: Query<&Actions>,
    camera_q: Query<(&FollowCamera, &Transform), Without<Player>>,

    time: Res<Time>,
) {
    let delta = time.delta_secs();

    let (settings, mut transform, mut velocity, mut player, is_grounded, coyote_time) =
        match player_q.single_mut() {
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
    if actions.pressed(Action::MoveUp) {
        movement += forward;
    }
    if actions.pressed(Action::MoveDown) {
        movement -= forward;
    }
    if actions.pressed(Action::MoveLeft) {
        movement -= right;
    }
    if actions.pressed(Action::MoveRight) {
        movement += right;
    }

    if movement != Vec3::ZERO {
        movement = movement.normalize();
        velocity.0.x = movement.x * settings.speed;
        velocity.0.z = movement.z * settings.speed;
        let target_rot = Quat::from_rotation_y((movement.x).atan2(movement.z));
        transform.rotation = transform.rotation.slerp(target_rot, 10.0 * delta);
    } else {
        velocity.0.x = 0.0;
        velocity.0.z = 0.0;
    }

    if actions.pressed(Action::Jump) && is_grounded {
        velocity.0.y = settings.jump_impulse;
    }
    let moving = actions.pressed(Action::MoveUp)
        || actions.pressed(Action::MoveDown)
        || actions.pressed(Action::MoveLeft)
        || actions.pressed(Action::MoveRight);
    let effectively_grounded = is_grounded || coyote_time.timer > 0.0;

    player.state = if moving && effectively_grounded {
        PlayerState::Running
    } else if !effectively_grounded {
        PlayerState::Jumping
    } else {
        PlayerState::Idle
    };
}

pub fn move_and_slide_player(
    mut query: Query<
        (
            Entity,
            &GroundDetection,
            &mut Transform,
            &mut LinearVelocity,
            &Collider,
            &mut GroundContact,
        ),
        With<PlayerController>,
    >,
    move_and_slide: MoveAndSlide,
    time: Res<Time>,
) {
    for (entity, ground, mut transform, mut lin_vel, collider, mut contact) in &mut query {
        let mut hit_ground_or_ceiling = false;
        let up = *transform.up();
        let mut touching_ground = false;

        let MoveAndSlideOutput {
            position: new_position,
            projected_velocity,
        } = move_and_slide.move_and_slide(
            collider,
            transform.translation,
            transform.rotation,
            lin_vel.0,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &SpatialQueryFilter::from_excluded_entities([entity]),
            |hit| {
                let angle = up.angle_between(**hit.normal);
                let is_ground = angle <= ground.max_angle;
                let is_ceiling = is_ground && up.dot(**hit.normal) < 0.0;
                if is_ground || is_ceiling {
                    hit_ground_or_ceiling = true;
                }
                if is_ground {
                    touching_ground = true;
                }
                MoveAndSlideHitResponse::Accept
            },
        );
        transform.translation = new_position;
        contact.touching_ground = touching_ground;

        if hit_ground_or_ceiling {
            let velocity_along_up = lin_vel.dot(up);
            let new_velocity_along_up = projected_velocity.dot(up);
            lin_vel.0 += (new_velocity_along_up - velocity_along_up) * up;
        }
    }
}
pub fn update_coyote_time(
    time: Res<Time>,
    mut query: Query<(&mut CoyoteTime, Has<Grounded>, &LinearVelocity)>,
) {
    let delta = time.delta_secs();
    for (mut coyote, is_grounded, velocity) in &mut query {
        if is_grounded {
            coyote.timer = COYOTE_DURATION;
        } else {
            coyote.timer = (coyote.timer - delta).max(0.0);
            /*  info!(
                "Timer: {:.3} | grounded: {} | vel.y: {:.3}",
                coyote.timer, is_grounded, velocity.y
            );*/
        }
    }
}
