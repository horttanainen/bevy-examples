use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::Buffer, renderer::RenderQueue},
};

use crate::camera::MainCamera;

#[derive(Resource, Default)]
pub struct CueBallPosition(Vec2);

impl ExtractResource for CueBallPosition {
    type Source = CueBallPosition;

    fn extract_resource(position: &Self::Source) -> Self {
        CueBallPosition(position.0)
    }
}

#[derive(Component)]
pub struct CueBall;

#[derive(Resource)]
pub struct CueBallBuffer(pub Buffer);

pub fn track_cue_ball_position(
    cue_ball: Query<(&mut Transform, With<CueBall>)>,
    mut cue_ball_position: ResMut<CueBallPosition>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();

    let (transform, _) = cue_ball.single();

    let view_pos = camera
        .world_to_viewport(camera_transform, transform.translation)
        .unwrap();
    cue_ball_position.0.x = view_pos.x;
    cue_ball_position.0.y = view_pos.y;
}

pub fn prepare_cue_ball(
    cue_ball: Res<CueBallPosition>,
    cue_ball_meta: ResMut<CueBallBuffer>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &cue_ball_meta.0,
        0,
        bevy::core::cast_slice(&[cue_ball.0.x, cue_ball.0.y]),
    );
}
