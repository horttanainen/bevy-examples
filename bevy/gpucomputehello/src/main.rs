
use bevy::{
    prelude::*,
    window::*,
};

use config::CONFIG;
use image::{create_texture, GpuComputeImage};
use plugin::GpuComputePlugin;

mod node;
mod config;
mod pipeline;
mod plugin;
mod time;
mod image;
mod bind_group;

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
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = create_texture(&mut images);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(CONFIG.size.0 as f32 * 3.0, CONFIG.size.0 as f32 * 3.0)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(GpuComputeImage(image));
}

