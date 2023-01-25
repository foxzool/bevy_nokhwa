use bevy::prelude::*;
use bevy_nokhwa::BevyNokhwaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyNokhwaPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .run();
}
