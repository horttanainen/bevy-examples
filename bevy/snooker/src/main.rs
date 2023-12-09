use ball::{track_ball_positions, Ball, BallPositions};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::*};
use bevy_rapier3d::prelude::{Collider, NoUserData, RapierPhysicsPlugin, RigidBody};

use camera::MainCamera;
use config::CONFIG;
use cue_ball::{track_cue_ball_position, CueBall, CueBallPosition};
use cursor::handle_cursor;
use debug::draw_viewport_rect;
use image::{create_texture, GpuComputeImage};
use movement::move_cue_ball;
use plugin::GpuComputePlugin;
use pocket::Pocket;
use rand::random;
use wall::Wall;

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
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_cue_ball,
                handle_cursor,
                track_cue_ball_position,
                track_ball_positions,
                draw_viewport_rect,
            ),
        )
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
            custom_size: Some(Vec2::new(CONFIG.size.0 as f32, CONFIG.size.1 as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.insert_resource(GpuComputeImage(image));
    commands.insert_resource(CueBallPosition::default());
    commands.insert_resource(BallPositions::default());

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(CONFIG.ball_radius))
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(CONFIG.ball_radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
            ..default()
        })
        .insert(CueBall);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(CONFIG.size.0 as f32, CONFIG.wall_width, 1.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(CONFIG.wall_color)),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                (CONFIG.wall_width / 2.0) - (CONFIG.size.1 as f32) / 2.0,
                10.0,
            )),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: CONFIG.wall_color,
                custom_size: Some(Vec2::new(CONFIG.size.0 as f32, CONFIG.wall_width)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -(CONFIG.wall_width / 2.0) + (CONFIG.size.1 as f32) / 2.0,
                10.0,
            )),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(CONFIG.wall_width, CONFIG.size.1 as f32, 1.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(CONFIG.wall_color)),
            transform: Transform::from_translation(Vec3::new(
                -(CONFIG.wall_width / 2.0) + (CONFIG.size.0 as f32) / 2.0,
                0.0,
                10.0,
            )),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(CONFIG.wall_width, CONFIG.size.1 as f32, 1.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(CONFIG.wall_color)),
            transform: Transform::from_translation(Vec3::new(
                (CONFIG.wall_width / 2.0) - (CONFIG.size.0 as f32) / 2.0,
                0.0,
                10.0,
            )),
            ..default()
        },
        Wall,
    ));

    for x in [
        (CONFIG.wall_width / 2.0) - (CONFIG.size.0 as f32) / 2.0,
        0.0,
        -(CONFIG.wall_width / 2.0) + (CONFIG.size.0 as f32) / 2.0,
    ] {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(CONFIG.pocket_radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    x,
                    (CONFIG.wall_width / 2.0) - (CONFIG.size.1 as f32) / 2.0,
                    15.0,
                )),
                ..default()
            },
            Pocket,
        ));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(CONFIG.pocket_radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    x,
                    -(CONFIG.wall_width / 2.0) + (CONFIG.size.1 as f32) / 2.0,
                    15.0,
                )),
                ..default()
            },
            Pocket,
        ));
    }

    for _ in 0..CONFIG.number_of_balls {
        let mut position = Vec3::new(random::<f32>() - 0.5, random::<f32>() - 0.5, 0.) * 500.;
        position.z = 20.;

        commands
            .spawn(RigidBody::Fixed)
            .insert(Collider::ball(CONFIG.ball_radius))
            .insert(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(CONFIG.ball_radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(Ball);
    }
}
