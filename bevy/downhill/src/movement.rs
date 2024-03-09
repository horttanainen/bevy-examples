use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{directions::Direction, player::Player, velocity::NonZeroSurfaceVelocity};

pub fn move_player(
    mut ext_forces_q: Query<&mut ExternalForce, With<Player>>,
    input: Res<Input<KeyCode>>,
    surface_velocity: Res<NonZeroSurfaceVelocity>,
    direction: Res<Direction>,
) {
    let mut external_force = ext_forces_q.single_mut();

    let perpendicular = direction
        .player_up
        .cross(surface_velocity.0)
        .normalize_or_zero();

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
