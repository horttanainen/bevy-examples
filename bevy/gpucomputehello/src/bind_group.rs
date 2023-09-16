use bevy::{
    prelude::*,
    render::{render_asset::*, render_resource::*, renderer::*},
};

use crate::{
    ball::BallBuffer, cue_ball::CueBallBuffer, image::GpuComputeImage,
    pipeline::GpuComputePipeline, time::TimeMeta,
};

#[derive(Resource)]
pub struct GpuComputeBindGroup(pub BindGroup);

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<GpuComputePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    hello_image: Res<GpuComputeImage>,
    render_device: Res<RenderDevice>,
    time_meta: ResMut<TimeMeta>,
    cue_ball_buffer: ResMut<CueBallBuffer>,
    ball_buffer: ResMut<BallBuffer>,
) {
    let view = &gpu_images[&hello_image.0];
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.texture_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&view.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: time_meta.buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: cue_ball_buffer.0.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 3,
                resource: ball_buffer.0.as_entire_binding(),
            },
        ],
    });
    commands.insert_resource(GpuComputeBindGroup(bind_group));
}
