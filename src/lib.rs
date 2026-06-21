use crate::background::{
    handle_background_image, prepare_background, render_background, BackgroundBindGroup,
    BackgroundImage, BackgroundPipeline,
};
use bevy::core_pipeline::{Core2d, Core2dSystems, Core3d, Core3dSystems};
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_resource::SpecializedRenderPipelines;
use bevy::render::{GpuResourceAppExt, RenderApp};

pub use nokhwa;

mod background;
pub mod camera;

pub struct BevyNokhwaPlugin;

impl Plugin for BevyNokhwaPlugin {
    fn build(&self, app: &mut App) {
        // Register the embedded webcam shader so it can be loaded as a `Handle<Shader>`.
        bevy::asset::embedded_asset!(app, "shader.wgsl");

        app.insert_resource(BackgroundImage(Image::default()))
            .add_plugins(ExtractResourcePlugin::<BackgroundImage>::default())
            .add_systems(Update, handle_background_image);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<BackgroundBindGroup>()
            .init_resource::<SpecializedRenderPipelines<BackgroundPipeline>>()
            .init_gpu_resource::<BackgroundPipeline>()
            // Draw the webcam background before the main pass for both 2d and 3d cameras.
            .add_systems(
                Core2d,
                (prepare_background, render_background)
                    .chain()
                    .before(Core2dSystems::MainPass),
            )
            .add_systems(
                Core3d,
                (prepare_background, render_background)
                    .chain()
                    .before(Core3dSystems::MainPass),
            );
    }
}
