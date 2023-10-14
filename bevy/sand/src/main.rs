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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Setup,
    Playing,
}

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
        .add_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Setup), setup)
        .add_systems(OnExit(GameState::Playing), teardown)
        .add_systems(
            Update,
            (draw_viewport_rect).run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, mut next_state: ResMut<NextState<GameState>>) {
    let image = create_texture(&mut images);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(CONFIG.size.0 as f32, CONFIG.size.1 as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.insert_resource(SandImage(image));
    next_state.set(GameState::Playing);
}

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
