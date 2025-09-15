use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // Red cube
    commands.spawn(PbrBundle {
        mesh: Mesh::from(shape::Cube { size: 2.0 }),
        material: StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(-3.0, 1.0, 0.0),
        ..Default::default()
    });
    // Green sphere
    commands.spawn(PbrBundle {
        mesh: Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 4 }),
        material: StandardMaterial {
            base_color: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });
    // Blue torus
    commands.spawn(PbrBundle {
        mesh: Mesh::from(shape::Torus { radius: 1.0, ring_radius: 0.3, subdivisions_segments: 16, subdivisions_sides: 32 }),
        material: StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        ..Default::default()
    });
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: Mesh::from(shape::Plane { size: 20.0 }),
        material: StandardMaterial {
            base_color: Color::rgb(0.3, 0.3, 0.3),
            ..Default::default()
        },
        ..Default::default()
    });
}
