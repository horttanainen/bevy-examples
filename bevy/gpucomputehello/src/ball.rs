use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::Buffer, renderer::RenderQueue},
};

use crate::camera::MainCamera;

#[derive(Resource, Default)]
pub struct BallPosition(Vec2);

impl ExtractResource for BallPosition {
    type Source = BallPosition;

    fn extract_resource(position: &Self::Source) -> Self {
        BallPosition(position.0)
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Resource)]
pub struct BallBuffer(pub Buffer);

pub fn track_ball_position(
    ball: Query<(&mut Transform, With<Ball>)>,
    mut ball_position: ResMut<BallPosition>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();

    let (transform, _) = ball.single();

    let view_pos = camera
        .world_to_viewport(camera_transform, transform.translation)
        .unwrap();
    ball_position.0.x = view_pos.x;
    ball_position.0.y = view_pos.y;
}

pub fn prepare_ball(
    ball: Res<BallPosition>,
    ball_buffer: ResMut<BallBuffer>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &ball_buffer.0,
        0,
        bevy::core::cast_slice(&[ball.0.x, ball.0.y]),
    );
}
