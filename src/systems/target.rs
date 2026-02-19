use bevy::prelude::*;
use bevy::picking::pointer::PointerInteraction;

use crate::player::ennemy_test::Ennemy;

#[derive(Resource, Default)]
pub struct CurrentTarget {
    pub entity: Option<Entity>,
}

pub fn on_enemy_clicked(
    click: On<Pointer<Click>>,
    mut current_target: ResMut<CurrentTarget>,
) {
    if click.event().button == PointerButton::Primary {
        current_target.entity = Some(click.event_target());
    }
}

pub fn highlight_target(
    current_target: Res<CurrentTarget>,
    enemy_query: Query<(Entity, &MeshMaterial3d<StandardMaterial>), With<Ennemy>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, material_handle) in enemy_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            if current_target.entity == Some(entity) {
                material.base_color = Color::srgb(1.0, 0.8, 0.0); // yellow/orange highlight
            } else {
                material.base_color = Color::srgb(1.0, 0.0, 1.0); // default magenta
            }
        }
    }
}

pub fn clear_target_on_miss(
    mouse: Res<ButtonInput<MouseButton>>,
    mut current_target: ResMut<CurrentTarget>,
    pointers: Query<&PointerInteraction>,
    enemies: Query<Entity, With<Ennemy>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // Check if pointer is hovering over an enemy (not just the ground)
        let hovering_enemy = pointers.iter().any(|interaction| {
            interaction.iter().any(|(entity, _)| enemies.get(*entity).is_ok())
        });
        if !hovering_enemy {
            current_target.entity = None;
        }
    }
}
