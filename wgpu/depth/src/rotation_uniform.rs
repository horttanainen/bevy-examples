use cgmath::SquareMatrix;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RotationUniform {
    rotation: [[f32; 3]; 3],
    degrees: f32,
    _padding2: u32,
    _padding3: u32,
}

impl RotationUniform {
    pub fn new() -> Self {
        Self {
            rotation: cgmath::Matrix3::identity().into(),
            degrees: 0.0,
            _padding2: 0,
            _padding3: 0,
        }
    }

    pub fn turn(&mut self, angle: f32) {
        self.degrees += angle;

        self.rotation = cgmath::Matrix3::from_angle_x(cgmath::Deg(self.degrees)).into();
    }
}
