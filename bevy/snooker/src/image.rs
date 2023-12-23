use bevy::{
    prelude::*,
    render::{extract_resource::*, render_resource::*, texture::*},
};

use crate::config::CONFIG;

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct GpuComputeImage(pub Handle<Image>);

pub fn create_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d {
            width: CONFIG.table_size.x as u32,
            height: CONFIG.table_size.y as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    image.sampler = ImageSampler::nearest();
    images.add(image)
}

pub fn setup_image(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let image = create_texture(&mut images);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(CONFIG.table_size.x as f32, CONFIG.table_size.y as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });
    commands.insert_resource(GpuComputeImage(image));
}
