use bevy::{
    render::{*, render_resource::*, extract_resource::*, renderer::*, render_graph::*}, prelude::*
};

use crate::{pipeline::GpuComputePipeline, time::{TimeMeta, ExtractedTime, prepare_time}, image::GpuComputeImage, bind_group::queue_bind_group, node::GpuComputeNode};

pub struct GpuComputePlugin;

impl Plugin for GpuComputePlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins(ExtractResourcePlugin::<GpuComputeImage>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTime>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));
        render_app.add_systems(Render, prepare_time.in_set(RenderSet::Prepare));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("hello_node", GpuComputeNode::default());
        render_graph.add_node_edge("hello_node", bevy::render::main_graph::node::CAMERA_DRIVER);
    }

    fn finish(&self, app: &mut App) {
        let render_device = app.world.resource::<RenderDevice>();

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: std::mem::size_of::<f32>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<GpuComputePipeline>()
            .insert_resource(TimeMeta {
                buffer,
            });
    }
}

