use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::config::CONFIG;

#[derive(Component)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Collider::capsule_y(0.5, 0.5))
        .insert(ColliderMassProperties::Mass(100.0))
        .insert(GravityScale(0.0))
        .insert(ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
        .insert(Restitution {
            coefficient: 0.7,
            ..default()
        })
        .insert(PbrBundle {
            mesh: meshes.add(shape::Capsule::default().into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(CONFIG.planet_center + CONFIG.planet_radius + 20.0),
            ..default()
        })
        .insert(Player);
}


