use ball::Ball;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::*};

use config::CONFIG;
use cue_ball::{CueBall, track_cue_ball_position, CueBallPosition, MainCamera};
use image::{create_texture, GpuComputeImage};
use movement::move_cue_ball;
use plugin::GpuComputePlugin;

mod bind_group;
mod config;
mod image;
mod node;
mod pipeline;
mod plugin;
mod time;
mod movement;
mod cue_ball;
mod ball;

fn main() {
    let res = WindowResolution::new(CONFIG.size.0 as f32, CONFIG.size.1 as f32);

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
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_cue_ball, track_cue_ball_position))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = create_texture(&mut images);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(
                CONFIG.size.0 as f32 * 3.0,
                CONFIG.size.0 as f32 * 3.0,
            )),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.insert_resource(GpuComputeImage(image));
    commands.insert_resource(CueBallPosition::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(CONFIG.ball_radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::ONE),
            ..default()
        },
        CueBall,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(CONFIG.ball_radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(Vec3::ONE * 50.),
            ..default()
        },
        Ball,
    ));
}
