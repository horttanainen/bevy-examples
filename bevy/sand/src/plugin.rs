use bevy::{
    prelude::*,
    render::{extract_resource::*, render_graph::*, render_resource::*, renderer::*, *},
};

use crate::{
    bind_group::queue_bind_group,
    image::SandImage,
    node::SandNode,
    pipeline::SandPipeline,
    time::{prepare_time, ExtractedTime, TimeMeta}, config::CONFIG,
};

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<SandImage>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTime>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));
        render_app.add_systems(Render, prepare_time.in_set(RenderSet::Prepare));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("sand_node", SandNode::default());
        render_graph.add_node_edge("sand_node", bevy::render::main_graph::node::CAMERA_DRIVER);
    }

    fn finish(&self, app: &mut App) {
        let render_device = app.world.resource::<RenderDevice>();

        let time_buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: std::mem::size_of::<f32>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<SandPipeline>()
            .insert_resource(TimeMeta {
                buffer: time_buffer,
            });
    }
}
