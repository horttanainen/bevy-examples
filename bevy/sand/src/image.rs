
use bevy::{
    prelude::*,
    render::{render_resource::*, texture::*, extract_resource::*, },
};

use crate::config::CONFIG;


#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct SandImage(pub Handle<Image>);

pub fn create_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d {
            width: CONFIG.size.0,
            height: CONFIG.size.1,
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

