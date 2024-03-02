use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{player::Player, planet::Planet, camera::MainCamera};

#[derive(Resource, Default)]
pub struct SurfaceVelocity(pub Vec3);

#[derive(Resource, Default)]
pub struct NonZeroSurfaceVelocity(pub Vec3);

pub fn setup_surface_velocity(
    mut commands: Commands,
) {
    commands.insert_resource(SurfaceVelocity::default());
    commands.insert_resource(NonZeroSurfaceVelocity::default());
}

pub fn calculate_surface_velocity(
    camera_q: Query<&Transform, With<MainCamera>>,
    player_q: Query<&Velocity, (With<Player>, Without<MainCamera>)>,
    planet_center_q: Query<&Transform, (With<Planet>, Without<MainCamera>)>,
    mut surface_velocity: ResMut<SurfaceVelocity>,
    mut non_zero_surface_velocity: ResMut<NonZeroSurfaceVelocity>
) {
    let transform = camera_q.single();
    let player_velocity = player_q.single();
    let planet_center = planet_center_q.single();

    let up = (transform.translation - planet_center.translation).normalize();
    let (planet_tangent_1, planet_tangent_2) = up.any_orthonormal_pair();

    let player_surface_velocity_1 = player_velocity.linvel.project_onto(planet_tangent_1);
    let player_surface_velocity_2 = player_velocity.linvel.project_onto(planet_tangent_2);
    let player_surface_velocity = player_surface_velocity_1 + player_surface_velocity_2;

    surface_velocity.0 = player_surface_velocity;
    if player_surface_velocity.length() > 0.1 {
        non_zero_surface_velocity.0 = player_surface_velocity.normalize();
    }
}
