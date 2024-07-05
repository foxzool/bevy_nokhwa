[![crates.io](https://img.shields.io/crates/v/bevy_nokhwa)](https://crates.io/crates/bevy_nokhwa)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![crates.io](https://img.shields.io/crates/d/bevy_nokhwa)](https://crates.io/crates/bevy_nokhwa)
[![CI](https://github.com/foxzool/bevy_nokhwa/workflows/CI/badge.svg)](https://github.com/foxzool/bevy_nokhwa/actions)
[![Documentation](https://docs.rs/bevy_nokhwa/badge.svg)](https://docs.rs/bevy_nokhwa)

# bevy_nokhwa

---

<img src="https://user-images.githubusercontent.com/217027/214884000-408ee6ce-ba88-4b2e-bb24-8fd7acadee90.png" width="640" height="480" alt="preview">

A bevy plugin using [nokhawa](https://github.com/l1npengtul/nokhwa).

This plugin allows you to render Camera Capture at background.

## Showcase

```rust
use bevy::prelude::*;
use bevy_nokhwa::BevyNokhwaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyNokhwaPlugin)
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
                // IMPORTANT! Need to set clear_color to None
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
}
```

# Support

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

| bevy | bevy_nokhwa |
|------|-------------|
| 0.14 | 0.6         |
| 0.13 | 0.5         |
| 0.12 | 0.4         |
| 0.11 | 0.3         |
| 0.10 | 0.2         |
| 0.9  | 0.1         |

# Licensing

The project is under dual license MIT and Apache 2.0, so join to your hearts content, just remember the license
agreements.

# Contributing

Yes this project is still very much WIP, so PRs are very welcome
