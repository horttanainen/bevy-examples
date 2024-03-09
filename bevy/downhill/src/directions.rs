use bevy::prelude::*;

use crate::{config::CONFIG, planet::Planet, player::Player, velocity::NonZeroSurfaceVelocity};

#[derive(Resource, Default)]
pub struct Direction {
    pub player_up: Vec3,
    pub planet_tangent_1: Vec3,
    pub planet_tangent_2: Vec3,
    pub center_of_gravity: Vec3,
    pub planet_center: Vec3,
}

pub fn setup_directions(mut commands: Commands) {
    commands.insert_resource(Direction::default());
}

pub fn calculate_directions(
    player_q: Query<&Transform, With<Player>>,
    planet_center_q: Query<&Transform, With<Planet>>,
    mut direction: ResMut<Direction>,
    surface_velocity: Res<NonZeroSurfaceVelocity>,
) {
    let player_transform = player_q.single();
    let planet_center = planet_center_q.single();

    let up = (player_transform.translation - planet_center.translation).normalize();
    let (planet_tangent_1, planet_tangent_2) = up.any_orthonormal_pair();

    direction.player_up = up;
    direction.planet_tangent_1 = planet_tangent_1;
    direction.planet_tangent_2 = planet_tangent_2;
    direction.center_of_gravity =
        planet_center.translation + surface_velocity.0.normalize() * CONFIG.planet_radius / 2.;
    direction.planet_center = planet_center.translation;
}
