use bevy::prelude::*;

use crate::systems::health::Health;

#[derive(Component)]
pub struct Player {
    pub pos: Vec3,
    
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn du joueur
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Player {
            pos: Vec3::new(0.0, 1.0, 0.0),
        },
        Health::new(100.0)
    ));
}
