use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{directions::Direction, player::Player};

#[derive(Resource, Default)]
pub struct SurfaceVelocity(pub Vec3);

#[derive(Resource, Default)]
pub struct NonZeroSurfaceVelocity(pub Vec3);

pub fn setup_surface_velocity(mut commands: Commands) {
    commands.insert_resource(SurfaceVelocity(Vec3::ONE));
    commands.insert_resource(NonZeroSurfaceVelocity(Vec3::ONE));
}

pub fn calculate_surface_velocity(
    player_q: Query<&Velocity, With<Player>>,
    direction: Res<Direction>,
    mut surface_velocity: ResMut<SurfaceVelocity>,
    mut non_zero_surface_velocity: ResMut<NonZeroSurfaceVelocity>,
) {
    let player_velocity = player_q.single();

    let player_surface_velocity_1 = player_velocity
        .linvel
        .project_onto(direction.planet_tangent_1);
    let player_surface_velocity_2 = player_velocity
        .linvel
        .project_onto(direction.planet_tangent_2);
    let player_surface_velocity = player_surface_velocity_1 + player_surface_velocity_2;

    surface_velocity.0 = player_surface_velocity;
    if player_surface_velocity.length() > 0.1 {
        non_zero_surface_velocity.0 = player_surface_velocity.normalize();
    }
}
