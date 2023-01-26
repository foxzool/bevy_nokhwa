use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_nokhwa::camera::BackgroundCamera;
use bevy_nokhwa::nokhwa::utils::ApiBackend;
use bevy_nokhwa::nokhwa::utils::FrameFormat;
use bevy_nokhwa::nokhwa::utils::{CameraFormat, RequestedFormatType, Resolution};
use bevy_nokhwa::BevyNokhwaPlugin;
use nokhwa::utils::CameraIndex;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "BevyNokhwa".to_string(),
                width: 1280.,
                height: 960.,
                ..default()
            },
            ..default()
        }))
        .add_plugin(BevyNokhwaPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        // auto find camera and use highest resolution
        // .insert(BackgroundCamera::auto())
        .insert(BackgroundCamera::new(
            ApiBackend::Auto,
            Some(CameraIndex::Index(0)),
            Some(RequestedFormatType::Closest(CameraFormat::new(
                Resolution::new(640, 480),
                FrameFormat::MJPEG,
                30,
            ))),
        ));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::SEA_GREEN,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}
