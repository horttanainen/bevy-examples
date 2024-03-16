use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use camera::move_camera;
use camera::setup_camera;
use directions::calculate_directions;
use directions::setup_directions;
use gravity::apply_gravity;
use lighting::setup_lighting;
use movement::move_player;
use planet::setup_planet;
use player::setup_player;
use stars::setup_stars;
use velocity::calculate_surface_velocity;
use velocity::setup_surface_velocity;

mod camera;
mod config;
mod directions;
mod gravity;
mod lighting;
mod movement;
mod planet;
mod player;
mod velocity;
mod stars;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_surface_velocity)
        .add_systems(Startup, setup_directions)
        .add_systems(Startup, setup_planet)
        .add_systems(Startup, setup_stars)
        .add_systems(Startup, setup_player)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_lighting)
        .add_systems(
            Update,
            (
                move_player,
                calculate_directions,
                apply_gravity,
                calculate_surface_velocity,
                move_camera,
            ),
        )
        .run();
}
