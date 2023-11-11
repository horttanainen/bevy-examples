use bevy::{prelude::{Handle, Resource}, sprite::ColorMaterial};


#[derive(Resource, Default)]
pub struct StoredMaterials {
    pub yellow: Handle<ColorMaterial>
}
