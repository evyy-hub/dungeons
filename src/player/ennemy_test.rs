use bevy::prelude::*;

use crate::combat::combat_stats::CombatStats;
use crate::systems::health::Health;
use crate::systems::target::on_enemy_clicked;

#[derive(Component)]
pub struct Ennemy {}

pub fn spawn_ennemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 1.0),
                ..Default::default()
            })),
            Transform::from_xyz(0.0, 1.0, 0.0),
            Ennemy {},
            Health::new(300.0),
            CombatStats::new(15.0, 20.0, 0.05, 2.0),
        ))
        .observe(on_enemy_clicked);
    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 1.0),
                ..Default::default()
            })),
            Transform::from_xyz(10.0, 1.0, 0.0),
            Ennemy {},
            Health::new(1000.0),
            CombatStats::new(15.0, 20.0, 0.05, 2.0),
        ))
        .observe(on_enemy_clicked);
}
