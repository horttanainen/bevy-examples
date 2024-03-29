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
        .insert(Ccd::enabled())
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
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(1., 0., 0.) * CONFIG.planet_radius * 0.9),
            ..default()
        })
        .insert(Player);
}
