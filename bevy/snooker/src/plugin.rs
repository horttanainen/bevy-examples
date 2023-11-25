use bevy::{
    prelude::*,
    render::{extract_resource::*, render_graph::*, render_resource::*, renderer::*, *},
};

use crate::{
    ball::{prepare_balls, BallBuffer, BallPositions},
    bind_group::queue_bind_group,
    cue_ball::{prepare_cue_ball, CueBallBuffer, CueBallPosition},
    image::GpuComputeImage,
    node::GpuComputeNode,
    pipeline::GpuComputePipeline,
    time::{prepare_time, ExtractedTime, TimeMeta}, config::CONFIG,
};

pub struct GpuComputePlugin;

impl Plugin for GpuComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<GpuComputeImage>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTime>::default())
            .add_plugins(ExtractResourcePlugin::<CueBallPosition>::default())
            .add_plugins(ExtractResourcePlugin::<BallPositions>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));
        render_app.add_systems(Render, prepare_time.in_set(RenderSet::Prepare));
        render_app.add_systems(Render, prepare_cue_ball.in_set(RenderSet::Prepare));
        render_app.add_systems(Render, prepare_balls.in_set(RenderSet::Prepare));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("hello_node", GpuComputeNode::default());
        render_graph.add_node_edge("hello_node", bevy::render::main_graph::node::CAMERA_DRIVER);
    }

    fn finish(&self, app: &mut App) {
        let render_device = app.world.resource::<RenderDevice>();

        let time_buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: std::mem::size_of::<f32>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let cue_ball_buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: (std::mem::size_of::<f32>() * 2) as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let ball_buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: (std::mem::size_of::<Vec4>() * (CONFIG.number_of_balls as usize)) as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<GpuComputePipeline>()
            .insert_resource(TimeMeta {
                buffer: time_buffer,
            })
            .insert_resource(CueBallBuffer(cue_ball_buffer))
            .insert_resource(BallBuffer(ball_buffer));
    }
}
