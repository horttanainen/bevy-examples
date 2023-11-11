use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

use crate::{camera::MainCamera, material_storage::StoredMaterials};

pub fn handle_cursor(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    stored_materials: Res<StoredMaterials>,
    rapier_context: Res<RapierContext>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        eprintln!("Mouse clicked!!!");

        if let Some(entity) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .and_then(|ray| {
                rapier_context.cast_ray(
                    ray.origin,
                    ray.direction,
                    1000.0,
                    true,
                    QueryFilter::default(),
                )
            })
        {

            eprintln!("Found entity!!!");
            if let Some(mut entity_commands) = commands.get_entity(entity.0) {
                entity_commands.remove::<Handle<ColorMaterial>>();
                entity_commands.insert(stored_materials.yellow.clone());
            }
        }
    }
}
