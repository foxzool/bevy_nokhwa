use bevy::prelude::*;

use bevy_nokhwa::camera::BackgroundCamera;
use bevy_nokhwa::BevyNokhwaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BevyNokhwa".to_string(),
                resolution: [1280, 960].into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BevyNokhwaPlugin)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        // Let nokhwa pick a format the device actually supports
        // (AbsoluteHighestFrameRate). On Apple Silicon cameras, fixed MJPEG
        // 640x480@30 may be rejected with "Cannot fulfill request".
        .insert(BackgroundCamera::auto().unwrap());

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: bevy::color::palettes::css::SEA_GREEN.into(),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
