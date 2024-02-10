use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{config::CONFIG, planet::Planet};

pub fn apply_gravity(
    mut ext_forces_q: Query<(&mut ExternalForce, &Transform)>,
    planet_center_q: Query<&Transform, With<Planet>>,
) {
    let planet_center = planet_center_q.single();
    for (mut ext_force, transform) in ext_forces_q.iter_mut() {
        ext_force.force = (planet_center.translation - transform.translation).normalize()
            * CONFIG.gravity
            * CONFIG.player_mass
    }
}
