# bevy_nokhwa
<div align="left">
<a href="https://crates.io/crates/bevy_nokhwa"><img src="https://img.shields.io/crates/v/bevy_nokhwa" alt="link to crates.io"></a>
<a href="https://docs.rs/bevy_nokhwa"><img src="https://docs.rs/bevy_nokhwa/badge.svg" alt="link to docs.rs"></a>
<a href="https://github.com/foxzool/bevy_nokhwa/blob/master/LICENSE-MIT"><img src="https://img.shields.io/crates/l/bevy_nokhwa" alt="link to license"></a>
<a href="https://crates.io/crates/bevy_nokhwa"><img src="https://img.shields.io/crates/d/bevy_nokhwa" alt="downloads/link to crates.io"></a>   
<a href="https://github.com/foxzool/bevy_nokhwa"><img src="https://img.shields.io/github/stars/foxzool/bevy_nokhwa" alt="stars/github repo"></a>
<a href="https://github.com/foxzool/bevy_nokhwa/actions/workflows/master.yml"><img src="https://github.com/foxzool/bevy_nokhwa/actions/workflows/master.yml/badge.svg" alt="github actions"></a>
<a href="https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking"><img src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue" alt="tracking bevy release branch"></a>
</div>
</br>

---

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

|bevy| bevy_nokhwa |
|--|-------------|
|0.9| 0.1.X       |

# Licensing
The project is under dual license MIT and Apache 2.0, so joink to your hearts content, just remember the license agreements.

# Contributing
Yes this project is still very much WIP, so PRs are very welcome