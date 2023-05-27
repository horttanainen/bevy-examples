use crate::rotation_uniform::RotationUniform;
use wgpu::util::DeviceExt;

pub struct RotationState {
    pub rotation_uniform: RotationUniform,
    pub rotation_buffer: wgpu::Buffer,
    pub rotation_bind_group: wgpu::BindGroup,
    pub rotation_bind_group_layout: wgpu::BindGroupLayout,
}

impl RotationState {
    pub fn new(device: &wgpu::Device) -> Self {
        let rotation_uniform = RotationUniform::new();

        let rotation_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rotation Buffer"),
            contents: bytemuck::cast_slice(&[rotation_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let rotation_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("rotation_bind_group_layout"),
            });

        let rotation_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &rotation_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: rotation_buffer.as_entire_binding(),
            }],
            label: Some("rotation_binding_group"),
        });

        Self {
            rotation_buffer,
            rotation_bind_group_layout,
            rotation_bind_group,
            rotation_uniform,
        }
    }
}
