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
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyNokhwaPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
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
}
