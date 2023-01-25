use crate::background::{BackgroundNode, BackgroundPipeline, BACKGROUND_NODE};
use bevy::core_pipeline;
use bevy::prelude::*;

use bevy::render::render_graph::RenderGraph;
use bevy::render::RenderApp;

mod background;

pub struct BevyNokhwaPlugin;

impl Plugin for BevyNokhwaPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<BackgroundPipeline>();
        let background_node = BackgroundNode::new(&mut render_app.world);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        if let Some(graph_3d) = render_graph.get_sub_graph_mut(core_pipeline::core_3d::graph::NAME)
        {
            graph_3d.add_node(BACKGROUND_NODE, background_node);
            // render_graph.add_sub_graph(BACKGROUND_GRAPH, background_graph);

            // render_graph.add_node(BACKGROUND_PASS_DRIVER, BackgroundPassDriverNode);
            graph_3d
                .add_node_edge(
                    BACKGROUND_NODE,
                    core_pipeline::core_3d::graph::node::MAIN_PASS,
                )
                .unwrap();
        }
    }
}
