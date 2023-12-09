use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::Buffer, renderer::RenderQueue}, core::{Pod, Zeroable},
};

use crate::camera::MainCamera;

#[repr(C)]
#[derive(Pod, Copy, Clone, Default)]
pub struct BallStatus {
    position: [f32; 3],
    selected: i32
}

unsafe impl Zeroable for BallStatus {
    fn zeroed() -> Self {
        BallStatus {
            position: [0., 0., 0.],
            selected: 0
        }
    }
}

#[derive(Resource, Default)]
pub struct BallPositions(Vec<BallStatus>);

impl ExtractResource for BallPositions {
    type Source = BallPositions;

    fn extract_resource(positions: &Self::Source) -> Self {
        BallPositions(positions.0.to_vec())
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Resource)]
pub struct BallBuffer(pub Buffer);

pub fn track_ball_positions(
    balls: Query<(&mut Transform, With<Ball>)>,
    mut ball_positions: ResMut<BallPositions>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();
    ball_positions.0.clear();

    for (transform, _) in &balls {
        let view_pos = camera
            .world_to_viewport(camera_transform, transform.translation)
            .unwrap();
        ball_positions.0.push(BallStatus { position: [view_pos.x, view_pos.y, 0.], selected: 0});
    }
}

pub fn prepare_balls(
    ball_positions: Res<BallPositions>,
    ball_buffer: ResMut<BallBuffer>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &ball_buffer.0,
        0,
        bevy::core::cast_slice(&ball_positions.0),
    );
}
