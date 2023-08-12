use bevy::{prelude::*, render::{render_resource::Buffer, renderer::RenderQueue, extract_resource::ExtractResource}};

#[derive(Resource, Default)]
pub struct CueBallPosition {
    translation: Vec2
}

#[derive(Resource, Default)]
pub struct ExtractedCueBall {
    x: f32,
    y: f32
}

impl ExtractResource for ExtractedCueBall {
    type Source = CueBallPosition;

    fn extract_resource(position: &Self::Source) -> Self {
        ExtractedCueBall {
            x: position.translation.x,
            y: position.translation.y
        }
    }
}


#[derive(Component)]
pub struct CueBall;

#[derive(Resource)]
pub struct CueBallMeta {
    pub buffer: Buffer,
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub fn track_cue_ball_position(
    cue_ball: Query<(&mut Transform, With<CueBall>)>,
    mut cue_ball_position: ResMut<CueBallPosition>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
     let (camera, camera_transform) = camera_q.single();

    let (transform, _)  = cue_ball.single();

    let view_pos =  camera.world_to_viewport(camera_transform,transform.translation);
    cue_ball_position.translation = view_pos.unwrap();
}

pub fn prepare_cue_ball(
    cue_ball: Res<ExtractedCueBall>,
    cue_ball_meta: ResMut<CueBallMeta>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &cue_ball_meta.buffer,
        0,
        bevy::core::cast_slice(&[cue_ball.x, cue_ball.y]),
    );
}
