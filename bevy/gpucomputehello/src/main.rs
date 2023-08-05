use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
            BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, CachedComputePipelineId,
            CachedPipelineState, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d,
            PipelineCache, ShaderStages, StorageTextureAccess, TextureDimension, TextureFormat,
            TextureUsages, TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice, RenderQueue},
        texture::ImageSampler,
        Render, RenderApp, RenderSet,
    },
    window::WindowResolution,
};

const SIZE: (u32, u32) = (1280, 720);
const WORKGROUP_SIZE: u32 = 8;

fn main() {
    let res = WindowResolution::new(SIZE.0 as f32, SIZE.1 as f32);

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: res,
                    title: "GPU Compute Hello".to_string(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            HelloPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn create_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    image.sampler_descriptor = ImageSampler::nearest();
    images.add(image)
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = create_texture(&mut images);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32 * 3.0, SIZE.0 as f32 * 3.0)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(HelloImage(image));
}

pub struct HelloPlugin;

#[derive(Resource, Clone, Deref, ExtractResource)]
struct HelloImage(Handle<Image>);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins(ExtractResourcePlugin::<HelloImage>::default())
            .add_plugins(ExtractResourcePlugin::<ExtractedTime>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));
        render_app.add_systems(Render, prepare_time.in_set(RenderSet::Prepare));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("hello_node", HelloNode::default());
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
            .init_resource::<HelloPipeline>()
            .insert_resource(TimeMeta {
                buffer,
                bind_group: None,
            });
    }
}

#[derive(Resource)]
struct HelloImageBindGroup(BindGroup);

fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<HelloPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    hello_image: Res<HelloImage>,
    render_device: Res<RenderDevice>,
    time_meta: ResMut<TimeMeta>,
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
        ],
    });
    commands.insert_resource(HelloImageBindGroup(bind_group));
}

#[derive(Resource)]
pub struct HelloPipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for HelloPipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout = world.resource::<RenderDevice>().create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
                        },
                        count: None,
                    },
                ],
            },
        );
        let shader = world.resource::<AssetServer>().load("shaders/hello.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        HelloPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum HelloState {
    Loading,
    Init,
    Update,
}

struct HelloNode {
    state: HelloState,
}

impl Default for HelloNode {
    fn default() -> Self {
        Self {
            state: HelloState::Loading,
        }
    }
}

impl render_graph::Node for HelloNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<HelloPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            HelloState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = HelloState::Init;
                }
            }
            HelloState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = HelloState::Update;
                }
            }
            HelloState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<HelloImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<HelloPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            HelloState::Loading => {}
            HelloState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            HelloState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}

#[derive(Resource, Default)]
struct ExtractedTime {
    seconds_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(time: &Self::Source) -> Self {
        ExtractedTime {
            seconds_since_startup: time.elapsed_seconds(),
        }
    }
}

#[derive(Resource)]
struct TimeMeta {
    buffer: Buffer,
    bind_group: Option<BindGroup>,
}

// write the extracted time into the corresponding uniform buffer
fn prepare_time(
    time: Res<ExtractedTime>,
    time_meta: ResMut<TimeMeta>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &time_meta.buffer,
        0,
        bevy::core::cast_slice(&[time.seconds_since_startup]),
    );
}
