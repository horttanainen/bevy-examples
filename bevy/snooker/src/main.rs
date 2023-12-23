use ball::{track_ball_positions, BallPositions, setup_balls};
use bevy::{prelude::*, window::*};
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

use camera::MainCamera;
use config::CONFIG;
use cue_ball::{track_cue_ball_position, CueBallPosition, setup_cue_ball};
use cursor::handle_cursor;
use debug::draw_viewport_rect;
use image::setup_image;
use movement::move_cue_ball;
use plugin::GpuComputePlugin;
use pocket::{setup_pockets, PocketPositions, track_pocket_selection};
use selection::highlight_selected;
use wall::setup_walls;

mod selection;
mod buffer_size;
mod ball;
mod bind_group;
mod camera;
mod config;
mod cue_ball;
mod cursor;
mod debug;
mod image;
mod movement;
mod node;
mod pipeline;
mod plugin;
mod pocket;
mod time;
mod wall;

fn main() {
    let res = WindowResolution::new(CONFIG.table_size.x as f32, CONFIG.table_size.y as f32);

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: res,
                    title: "GPU Compute Hello".to_string(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            GpuComputePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_systems(Startup, (setup, setup_image, setup_cue_ball, setup_balls, setup_walls, setup_pockets))
        .add_systems(
            Update,
            (
                move_cue_ball,
                handle_cursor,
                track_cue_ball_position,
                highlight_selected,
                track_ball_positions,
                track_pocket_selection,
                draw_viewport_rect,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.insert_resource(CueBallPosition::default());
    commands.insert_resource(BallPositions::default());
    commands.insert_resource(PocketPositions::default());
}
