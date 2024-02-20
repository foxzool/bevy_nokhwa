use crate::background::{
    handle_background_image, BackgroundImage, BackgroundNode, BackgroundNodeLabel,
    BackgroundPipeline,
};
use bevy::core_pipeline;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;

use bevy::render::render_graph::RenderGraph;
use bevy::render::RenderApp;
use image::RgbaImage;

pub use nokhwa;

mod background;
pub mod camera;

pub struct BevyNokhwaPlugin;

impl Plugin for BevyNokhwaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundImage(RgbaImage::new(640, 480)))
            .add_plugins(ExtractResourcePlugin::<BackgroundImage>::default())
            .add_systems(Update, handle_background_image);

        let render_app = app.sub_app_mut(RenderApp);

        let background_node_2d = BackgroundNode::new(&mut render_app.world);
        let background_node_3d = BackgroundNode::new(&mut render_app.world);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        if let Some(graph_2d) =
            render_graph.get_sub_graph_mut(core_pipeline::core_2d::graph::Core2d)
        {
            graph_2d.add_node(BackgroundNodeLabel, background_node_2d);

            graph_2d.add_node_edge(
                BackgroundNodeLabel,
                core_pipeline::core_2d::graph::Node2d::MainPass,
            );
        }

        if let Some(graph_3d) =
            render_graph.get_sub_graph_mut(core_pipeline::core_3d::graph::Core3d)
        {
            graph_3d.add_node(BackgroundNodeLabel, background_node_3d);

            graph_3d.add_node_edge(
                BackgroundNodeLabel,
                core_pipeline::core_3d::graph::Node3d::MainTransparentPass,
            );
        }
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<BackgroundPipeline>();
    }
}
