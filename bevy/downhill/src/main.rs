use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use camera::setup_camera;
use camera::move_camera;
use gravity::apply_gravity;
use lighting::setup_lighting;
use movement::move_player;
use planet::setup_planet;
use player::setup_player;
use velocity::calculate_surface_velocity;
use velocity::setup_surface_velocity;

mod config;
mod planet;
mod lighting;
mod player;
mod gravity;
mod camera;
mod movement;
mod velocity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_surface_velocity)
        .add_systems(Startup, setup_planet)
        .add_systems(Startup, setup_player)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_lighting)
        .add_systems(Update, (apply_gravity, move_camera, move_player, calculate_surface_velocity))
        .run();

}

