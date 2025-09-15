use bevy::prelude::*;

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
        mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..Default::default()
        }),
        transform: Transform::from_xyz(-3.0, 1.0, 0.0),
        ..Default::default()
    });

    // Green sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere::new(1.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });

    // Blue torus
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Torus::new(0.3, 1.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            ..Default::default()
        }),
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        ..Default::default()
    });

    // Ground plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Rectangle::new(20.0, 20.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            ..Default::default()
        }),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..Default::default()
    });
}