use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{player::Player, velocity::NonZeroSurfaceVelocity, planet::Planet};

pub fn move_player(
    mut ext_forces_q: Query<(&mut ExternalForce, &Transform), With<Player>>,
    planet_center_q: Query<&Transform, With<Planet>>,
    input: Res<Input<KeyCode>>,
    surface_velocity: Res<NonZeroSurfaceVelocity>,
) {
    let (mut external_force, player_transform) = ext_forces_q.single_mut();
    let planet_center = planet_center_q.single();

    let up = (player_transform.translation - planet_center.translation).normalize();
    let perpendicular = up.cross(surface_velocity.0).normalize();

    let force_multiplier = 1000.0;

    if input.pressed(KeyCode::W) {
        external_force.force += surface_velocity.0 * force_multiplier;
    } else if input.pressed(KeyCode::S) {
        external_force.force -= surface_velocity.0 * force_multiplier;
    }

    if input.pressed(KeyCode::D) {
        external_force.force -= perpendicular * force_multiplier;
    } else if input.pressed(KeyCode::A) {
        external_force.force += perpendicular * force_multiplier;
    }

}
