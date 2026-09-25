use avian3d::prelude::*;
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_mod_outline::OutlineVolume;

use crate::combat::combat_stats::CombatStats;
use crate::enemies::enemy_setup::Enemy;
use crate::enemies::enemy_test::GameLayer;
use crate::systems::death::Dead;

#[derive(Resource, Default)]
pub struct CurrentTarget {
    pub entity: Option<Entity>,
}

pub fn on_enemy_clicked(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut current_target: ResMut<CurrentTarget>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    spatial_query: SpatialQuery,
    parent_query: Query<&ChildOf>,
    stats_query: Query<&CombatStats>,
    dead_query: Query<Entity, With<Dead>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Ok(window) = q_window.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                for (camera, camera_transform) in q_camera.iter() {
                    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                        if let Some(hit) = spatial_query.cast_ray(
                            ray.origin,
                            ray.direction,
                            f32::MAX,
                            true,
                            &SpatialQueryFilter::default().with_mask(GameLayer::Enemy),
                        ) {
                            let mut target_entity = hit.entity;

                            // If hit entity has no CombatStats, search for parent
                            if stats_query.get(target_entity).is_err() {
                                if let Ok(parent) = parent_query.get(target_entity) {
                                    target_entity = parent.get();
                                    //println!(
                                    //    "target clicked (child): {:?}, using parent: {:?}",
                                    //    hit.entity, target_entity
                                    //);
                                }
                            } else {
                                //println!("target clicked: {:?}", target_entity);
                            }

                            if dead_query.get(target_entity).is_ok() {
                                current_target.entity = None;
                                continue;
                            }

                            current_target.entity = Some(target_entity);
                        }
                    }
                }
            }
        }
    }
}

pub fn highlight_target(
    current_target: Res<CurrentTarget>,
    mut enemy_query: Query<(Entity, &mut OutlineVolume), With<Enemy>>,
) {
    for (entity, mut outline) in enemy_query.iter_mut() {
        outline.visible = current_target.entity == Some(entity);
    }
}

pub fn clear_target_on_miss(
    mut current_target: ResMut<CurrentTarget>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    spatial_query: SpatialQuery,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Ok(window) = q_window.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                for (camera, camera_transform) in q_camera.iter() {
                    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                        // Raycast to check if clicking on enemy
                        let hit = spatial_query.cast_ray(
                            ray.origin,
                            ray.direction,
                            f32::MAX,
                            true,
                            &SpatialQueryFilter::default().with_mask(GameLayer::Enemy),
                        );

                        if hit.is_none() {
                            // Clear target if not clicking on enemy
                            current_target.entity = None;
                        }
                    }
                }
            }
        }
    }
}
