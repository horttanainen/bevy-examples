use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

use crate::{camera::MainCamera, selection::{select, Selection, de_select}};

pub fn handle_cursor(
    mut commands: Commands,
    selected_q: Query<&Selection>,
    buttons: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier_context: Res<RapierContext>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

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
            if let Ok(selection) = selected_q.get(entity.0) {
                if selection.selected {
                    de_select(&mut commands, entity.0);
                } else {
                    select(&mut commands, entity.0);
                }
            }
        }
    }
}
