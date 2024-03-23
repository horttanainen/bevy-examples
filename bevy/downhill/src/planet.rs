use bevy::{prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use bevy_rapier3d::{parry::shape::SharedShape, prelude::*};

use crate::config::CONFIG;

#[derive(Component)]
pub struct Planet;

pub fn setup_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball_shape = SharedShape::ball(CONFIG.planet_radius);
    let (vertices, indices) = ball_shape.as_ball().unwrap().to_trimesh(10, 10);

    let reversed_indices: Vec<[u32; 3]> = indices.into_iter().map(|x| [x[2], x[1], x[0]]).collect();

    let vertices_as_vec3: Vec<Vec3> = vertices.into_iter().map(|y| Vec3::new(y.x, y.y, y.z)).collect();

    let collider_reversed_incides = Collider::trimesh(vertices_as_vec3.clone(), reversed_indices.clone());

    let flattened_reversed_indices = Indices::U32(reversed_indices.into_iter().flatten().collect());

    let normals: Vec<Vec3> = vertices_as_vec3.clone().into_iter().map(|k| (Vec3::ZERO - k).normalize()).collect();

    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices_as_vec3
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals 
        )
        .with_indices(
            Some(flattened_reversed_indices)
        );

    commands
        .spawn(RigidBody::Fixed)
        .insert(collider_reversed_incides)
        .insert(ColliderMassProperties::Density(10.0))
        .insert(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::PINK,
                double_sided: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_translation(CONFIG.planet_center),
            ..default()
        })
        .insert(Planet);
}
