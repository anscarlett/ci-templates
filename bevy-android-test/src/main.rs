use bevy::prelude::*;
use bevy::render::mesh::shape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    // Camera
    commands.spawn((
        Camera3d,
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
    ));

    // Red cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..Default::default()
        }),
        transform: Transform::from_xyz(-3.0, 1.0, 0.0),
        ..Default::default()
    });

    // Green sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 4 })),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });

    // Blue torus
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Torus {
            radius: 1.0,
            ring_radius: 0.3,
            subdivisions_segments: 16,
            subdivisions_sides: 32,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            ..Default::default()
        }),
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        ..Default::default()
    });

    // Ground plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            ..Default::default()
        }),
        ..Default::default()
    });
}
