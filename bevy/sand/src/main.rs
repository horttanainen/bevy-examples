use bevy::{prelude::*, window::*};

use camera::MainCamera;
use config::CONFIG;
use debug::draw_viewport_rect;
use image::{create_texture, SandImage};
use plugin::SandPlugin;

mod bind_group;
mod camera;
mod config;
mod debug;
mod image;
mod node;
mod pipeline;
mod plugin;
mod time;

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
            SandPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
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
    commands.insert_resource(SandImage(image));

}
