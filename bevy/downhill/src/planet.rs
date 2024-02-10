use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::config::CONFIG;

#[derive(Component)]
pub struct Planet;

pub fn setup_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(CONFIG.planet_radius))
        .insert(ColliderMassProperties::Density(10.0))
        .insert(PbrBundle {
            mesh: meshes.add(shape::Icosphere{radius: CONFIG.planet_radius, subdivisions: 1}.try_into().unwrap()),
            material: materials.add(Color::SEA_GREEN.into()),
            transform: Transform::from_translation(CONFIG.planet_center),
            ..default()
        })
        .insert(Planet);
}
