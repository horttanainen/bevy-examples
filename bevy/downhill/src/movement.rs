use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{planet::Planet, player::Player};

pub fn move_player(
    mut ext_forces_q: Query<(&mut ExternalForce, &Velocity, &Transform), With<Player>>,
    planet_center_q: Query<&Transform, With<Planet>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut external_force, velocity, player_transform) = ext_forces_q.single_mut();
    let planet_center_transform = planet_center_q.single();

    let up = (player_transform.translation - planet_center_transform.translation).normalize();

    let quat = Quat::from_axis_angle(up, 0. * PI);

    let mut controlled_direction = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        controlled_direction.z = -1.;
    } else if input.pressed(KeyCode::S) {
        controlled_direction.z = 1.;
    }
    if input.pressed(KeyCode::D) {
        controlled_direction.x = 1.;
    } else if input.pressed(KeyCode::A) {
        controlled_direction.x = -1.;
    }
}
