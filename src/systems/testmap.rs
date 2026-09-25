use avian3d::{
    collision::collider::{Collider, ColliderConstructor, ColliderConstructorHierarchy},
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;

pub fn testmap(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
    ));
    commands.spawn((
        WorldAssetRoot(asset_server.load("test/scene.gltf#Scene0")),
        Transform::from_xyz(-10.0, 0.0, 0.0),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
    ));
    commands.spawn((
        WorldAssetRoot(asset_server.load("test/torch.gltf#Scene0")),
        Transform::from_xyz(10.0, 1.0, 0.0),
    ));
}
