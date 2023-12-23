use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_rapier3d::prelude::{RigidBody, Collider};

use crate::{config::CONFIG, selection::Selection};


#[derive(Component)]
pub struct Pocket;

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
