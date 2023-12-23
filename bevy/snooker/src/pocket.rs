use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::{extract_resource::ExtractResource, render_resource::Buffer, renderer::RenderQueue}};

use bevy_rapier3d::prelude::{RigidBody, Collider};
use bytemuck::{Zeroable, Pod};

use crate::{config::CONFIG, selection::Selection, camera::MainCamera};

#[repr(C)]
#[derive(Pod, Copy, Clone, Default)]
pub struct PocketStatus {
    position: [f32; 3],
    selected: i32
}

unsafe impl Zeroable for PocketStatus {
    fn zeroed() -> Self {
        PocketStatus {
            position: [0., 0., 0.],
            selected: 0
        }
    }
}

#[derive(Resource, Default)]
pub struct PocketPositions(Vec<PocketStatus>);

impl ExtractResource for PocketPositions {
    type Source = PocketPositions;

    fn extract_resource(positions: &Self::Source) -> Self {
        PocketPositions(positions.0.to_vec())
    }
}

#[derive(Component)]
pub struct Pocket;

#[derive(Resource)]
pub struct PocketBuffer(pub Buffer);

pub fn track_pocket_selection(
    pockets: Query<(&mut Transform, &Selection, With<Pocket>)>,
    mut pocket_positions: ResMut<PocketPositions>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();
    pocket_positions.0.clear();

    for (transform, selection, _) in &pockets {
        let view_pos = camera
            .world_to_viewport(camera_transform, transform.translation)
            .unwrap();
        pocket_positions.0.push(PocketStatus { position: [view_pos.x, view_pos.y, 0.], selected: selection.selected as i32});
    }
}

pub fn prepare_pockets(
    pocket_positions: Res<PocketPositions>,
    pocket_buffer: ResMut<PocketBuffer>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &pocket_buffer.0,
        0,
        bevy::core::cast_slice(&pocket_positions.0),
    );
}

pub fn setup_pockets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    for x in [
        (CONFIG.wall_width / 2.0) - (CONFIG.table_size.x as f32) / 2.0,
        0.0,
        -(CONFIG.wall_width / 2.0) + (CONFIG.table_size.x as f32) / 2.0,
    ] {
        commands
            .spawn(RigidBody::Fixed)
            .insert(Collider::ball(CONFIG.pocket_radius))
            .insert(
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(CONFIG.pocket_radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    x,
                    (CONFIG.wall_width / 2.0) - (CONFIG.table_size.y as f32) / 2.0,
                    15.0,
                )),
                ..default()
            }
        )
        .insert(Pocket)
        .insert(Selection { selected: false});

        commands
            .spawn(RigidBody::Fixed)
            .insert(Collider::ball(CONFIG.pocket_radius))
            .insert(
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(CONFIG.pocket_radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    x,
                    -(CONFIG.wall_width / 2.0) + (CONFIG.table_size.y as f32) / 2.0,
                    15.0,
                )),
                ..default()
            },
        )
        .insert(Pocket)
        .insert(Selection { selected: false});
    }

}
