use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::Buffer, renderer::RenderQueue}, core::{Pod, Zeroable}, sprite::MaterialMesh2dBundle,
};
use bevy_rapier3d::prelude::{RigidBody, Collider};
use rand::random;

use crate::{camera::MainCamera, config::CONFIG, selection::Selection};

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
    balls: Query<(&mut Transform, &Selection, With<Ball>)>,
    mut ball_positions: ResMut<BallPositions>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();
    ball_positions.0.clear();

    for (transform, selection, _) in &balls {
        let view_pos = camera
            .world_to_viewport(camera_transform, transform.translation)
            .unwrap();
        ball_positions.0.push(BallStatus { position: [view_pos.x, view_pos.y, 0.], selected: selection.selected as i32});
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

pub fn setup_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..CONFIG.number_of_balls {
        let mut position = Vec3::new(random::<f32>() - 0.5, random::<f32>() - 0.5, 0.) * 500.;
        position.z = 20.;

        commands
            .spawn(RigidBody::Fixed)
            .insert(Collider::ball(CONFIG.ball_radius))
            .insert(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(CONFIG.ball_radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(Ball)
            .insert(Selection { selected: false});
    }
}
