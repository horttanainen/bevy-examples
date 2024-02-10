use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{player::Player, planet::Planet};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50., 400.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);
}

pub fn move_camera(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    player_q: Query<(&Transform, &Velocity), (With<Player>, Without<MainCamera>)>,
    planet_center_q: Query<&Transform, (With<Planet>, Without<MainCamera>)>,
) {
    let mut transform = camera_q.single_mut();
    let (player_transform, player_velocity) = player_q.single();
    let planet_center = planet_center_q.single();

    let up = (transform.translation - planet_center.translation).normalize();
    let distance_to_player = Vec3::distance(transform.translation, player_transform.translation);
    let (planet_tangent_1, planet_tangent_2) = up.any_orthonormal_pair();

    let player_surface_velocity_1 = player_velocity.linvel.project_onto(planet_tangent_1);
    let player_surface_velocity_2 = player_velocity.linvel.project_onto(planet_tangent_2);
    let player_surface_velocity = player_surface_velocity_1 + player_surface_velocity_2;


    if player_surface_velocity.length() > 2.0 {
        let camera_behind_player = player_surface_velocity.normalize() * 10.0;
        transform.translation = transform.translation + transform.forward() * (distance_to_player - 10.) - camera_behind_player + up * 5.0;
    }
    transform.look_at(player_transform.translation, up);
}
