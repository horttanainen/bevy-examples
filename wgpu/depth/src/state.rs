use crate::camera_state::CameraState;
use crate::instance::Instance;
use crate::render_pipeline::create_render_pipeline;
use crate::rotation_state::RotationState;
use crate::texture;
use crate::texture_state::TextureState;
use crate::vertex::Vertex;

use wgpu::util::DeviceExt;

use winit::{event::*, window::Window};

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,

    pub color: wgpu::Color,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,

    pub rotation: RotationState,
    pub camera: CameraState,
    pub texture: TextureState,
    pub depth_texture: TextureState,
    pub render_pipeline: wgpu::RenderPipeline,
    pub shadow_toggle: bool,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(
        window: Window,
        instances: Vec<Instance>,
        diffuse_bytes: &[u8],
        indices: &[u16],
        vertices: &[Vertex],
    ) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::METAL,
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let camera = CameraState::new(&config, &device);

        let depth_texture = TextureState::create_depth(&config, &device);
        let texture = TextureState::create(&device, &queue, diffuse_bytes);

        let color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let rotation = RotationState::new(&device);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let render_pipeline =
            create_render_pipeline(&device, &shader, &config, &texture, &camera, &rotation);

        Self {
            color,
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            texture,
            depth_texture,
            camera,
            rotation,
            instances,
            instance_buffer,
            shadow_toggle: false,
        }
    }

    pub fn toggle_shadow(&mut self) {
        self.shadow_toggle = !self.shadow_toggle;

        if self.shadow_toggle {
            let shader = &self
                .device
                .create_shader_module(wgpu::include_wgsl!("shadow_shader.wgsl"));
            self.render_pipeline = create_render_pipeline(
                &self.device,
                shader,
                &self.config,
                &self.depth_texture,
                &self.camera,
                &self.rotation,
            );
        } else {
            let shader = &self
                .device
                .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
            self.render_pipeline = create_render_pipeline(
                &self.device,
                shader,
                &self.config,
                &self.texture,
                &self.camera,
                &self.rotation,
            );
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture.texture =
                texture::Texture::create_depth_texture_non_comparison_sampler(
                    &self.device,
                    &self.config,
                    "depth_texture",
                );
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera.camera_controller.process_events(event)
    }

    pub fn update(&mut self) {
        self.camera
            .camera_controller
            .update_camera(&mut self.camera.camera);
        self.camera
            .camera_uniform
            .update_view_proj(&self.camera.camera);
        self.queue.write_buffer(
            &self.camera.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.camera_uniform]),
        );

        self.instances.iter_mut().for_each(|instance| {
            instance.frame += 1;

            let translation = cgmath::Angle::sin(cgmath::Rad(instance.frame as f32 / 100.0));

            instance.position = instance.original_position * (1.0 + translation);
        });

        let instance_data = self
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        self.instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        self.rotation.rotation_uniform.turn(0.1);
        self.queue.write_buffer(
            &self.rotation.rotation_buffer,
            0,
            bytemuck::cast_slice(&[self.rotation.rotation_uniform]),
        );
    }

    pub fn render(&mut self, indices: &[u16]) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);

            if self.shadow_toggle {
                render_pass.set_bind_group(0, &self.depth_texture.bind_group, &[]);
            } else {
                render_pass.set_bind_group(0, &self.texture.bind_group, &[]);
            }
            render_pass.set_bind_group(1, &self.camera.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.rotation.rotation_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..self.instances.len() as _);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

