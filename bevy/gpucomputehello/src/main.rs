use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, texture::ImageSampler}, window::WindowResolution};

#[derive(Component, Default, Clone)]
pub struct TileSystem {
    pub rendered_texture: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::default(),
            title: "GPU Compute Hello".to_string(),
            resizable: false,
            ..default()
        }),
        ..default()}))
        .add_systems(Startup, setup)
        .run();
}

fn create_texture(res: &WindowResolution, images: &mut Assets<Image>) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d {
            width: res.physical_width(),
            height: res.physical_height(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    image.sampler_descriptor = ImageSampler::nearest();
    images.add(image)
}


fn setup(mut commands: Commands, window: Query<&Window>, mut images: ResMut<Assets<Image>>) {
    let window = window.single();
    let image = create_texture(&window.resolution, &mut images);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(window.resolution.physical_width() as f32 * 3.0, window.resolution.physical_height() as f32 * 3.0)),
                ..default()
            },
            texture: image.clone(),
            ..default()
        })
        .insert(TileSystem {
            rendered_texture: image,
        });

    commands.spawn(Camera2dBundle::default());
}


