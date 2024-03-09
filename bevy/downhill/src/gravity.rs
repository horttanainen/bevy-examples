use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{config::CONFIG, directions::Direction};

pub fn apply_gravity(
    mut ext_forces_q: Query<(&mut ExternalForce, &Transform)>,
    direction: Res<Direction>,
) {
    for (mut ext_force, transform) in ext_forces_q.iter_mut() {
        ext_force.force = (direction.center_of_gravity - transform.translation).normalize()
            * CONFIG.gravity
            * CONFIG.player_mass
    }
}
