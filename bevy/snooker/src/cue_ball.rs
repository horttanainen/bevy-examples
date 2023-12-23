use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::Buffer, renderer::RenderQueue}, sprite::MaterialMesh2dBundle,
};
use bevy_rapier3d::prelude::{RigidBody, Collider};

use crate::{camera::MainCamera, config::CONFIG, selection::Selection};

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

pub fn setup_cue_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(CONFIG.ball_radius))
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(CONFIG.ball_radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(CONFIG.cue_ball_starting_position),
            ..default()
        })
        .insert(CueBall)
        .insert(Selection { selected: false});
}
